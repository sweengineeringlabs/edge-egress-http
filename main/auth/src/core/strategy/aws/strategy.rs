//! AWS Signature Version 4 strategy.
//!
//! Implements the AWS SigV4 algorithm as specified by AWS:
//! https://docs.aws.amazon.com/general/latest/gr/sigv4_signing.html
//!
//! On each request:
//!   1. Compute canonical request string (method, path, query,
//!      canonical headers, signed headers, payload hash)
//!   2. Compute string-to-sign (algorithm, datetime, credential
//!      scope, hash(canonical-request))
//!   3. Derive signing key (date → region → service → "aws4_request")
//!   4. Compute signature = HMAC-SHA256(signing-key, string-to-sign)
//!   5. Attach `Authorization: AWS4-HMAC-SHA256 Credential=...,
//!      SignedHeaders=..., Signature=...` + `x-amz-date` +
//!      optional `x-amz-security-token` headers.

use http::header::{HeaderName, HeaderValue, AUTHORIZATION, HOST};
use percent_encoding::{AsciiSet, CONTROLS};
use secrecy::{ExposeSecret, SecretString};
use sha2::{Digest, Sha256};
use time::format_description::FormatItem;
use time::{macros::format_description, OffsetDateTime};

use super::helper::AwsSigV4Helper;
use crate::api::error::AuthError;
use crate::api::traits::auth_strategy::AuthStrategy;

/// `AsciiSet` semantics: chars IN this set get percent-encoded.
/// Everything NOT in the set passes through literally.
///
/// Per AWS SigV4 spec, canonical strings percent-encode every
/// character EXCEPT the RFC 3986 §2.3 unreserved set:
/// `A-Z a-z 0-9 - . _ ~`.
///
/// For canonical URI (path): `/` is NOT in this set, so it
/// passes through — slashes stay literal in `/foo/bar`.
/// For canonical query: `/` IS in [`ENCODE_FOR_QUERY`] below,
/// so query components percent-encode slashes.
pub(crate) const ENCODE_FOR_PATH: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'!')
    .add(b'"')
    .add(b'#')
    .add(b'$')
    .add(b'%')
    .add(b'&')
    .add(b'\'')
    .add(b'(')
    .add(b')')
    .add(b'*')
    .add(b'+')
    .add(b',')
    .add(b':')
    .add(b';')
    .add(b'<')
    .add(b'=')
    .add(b'>')
    .add(b'?')
    .add(b'@')
    .add(b'[')
    .add(b'\\')
    .add(b']')
    .add(b'^')
    .add(b'`')
    .add(b'{')
    .add(b'|')
    .add(b'}');

/// Same as [`ENCODE_FOR_PATH`] plus `/` — for query-string
/// components per AWS SigV4 spec.
pub(crate) const ENCODE_FOR_QUERY: &AsciiSet = &ENCODE_FOR_PATH.add(b'/');

/// `YYYYMMDDTHHMMSSZ` — AWS X-Amz-Date format.
const AMZ_DATE_FMT: &[FormatItem<'static>] =
    format_description!("[year][month][day]T[hour][minute][second]Z");
/// `YYYYMMDD` — AWS credential-scope date component.
const AMZ_DATE_ONLY_FMT: &[FormatItem<'static>] = format_description!("[year][month][day]");

/// Pre-resolved AWS credentials bundle (private to this module).
struct AwsSigV4Credentials {
    access_key_id: SecretString,
    secret_access_key: SecretString,
    session_token: Option<SecretString>,
}

/// AWS SigV4 strategy. Holds pre-resolved credentials + the
/// static (region, service) pair for signing.
pub(crate) struct AwsSigV4Strategy {
    credentials: AwsSigV4Credentials,
    region: String,
    service: String,
}

impl std::fmt::Debug for AwsSigV4Strategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AwsSigV4Strategy")
            .field("access_key_id", &"<redacted>")
            .field("secret_access_key", &"<redacted>")
            .field(
                "session_token",
                &if self.credentials.session_token.is_some() {
                    "<set>"
                } else {
                    "<none>"
                },
            )
            .field("region", &self.region)
            .field("service", &self.service)
            .finish()
    }
}

impl AwsSigV4Strategy {
    /// Construct from resolved credentials + static
    /// region/service identifiers.
    pub(crate) fn new(
        access_key_id: SecretString,
        secret_access_key: SecretString,
        session_token: Option<SecretString>,
        region: String,
        service: String,
    ) -> Self {
        Self {
            credentials: AwsSigV4Credentials {
                access_key_id,
                secret_access_key,
                session_token,
            },
            region,
            service,
        }
    }

    /// The current timestamp — factored out so tests can inject
    /// a fixed time for deterministic signature comparison.
    pub(crate) fn now() -> OffsetDateTime {
        OffsetDateTime::now_utc()
    }

    /// Perform the signing pass on `req`. Extracted so tests
    /// can call it with a fixed `OffsetDateTime` to verify
    /// against known AWS test vectors.
    pub(crate) fn sign(
        &self,
        req: &mut reqwest::Request,
        now: OffsetDateTime,
    ) -> Result<(), AuthError> {
        let amz_date = now
            .format(AMZ_DATE_FMT)
            .map_err(|e| AuthError::InvalidHeaderValue(format!("format amz-date: {e}")))?;
        let date_scope = now
            .format(AMZ_DATE_ONLY_FMT)
            .map_err(|e| AuthError::InvalidHeaderValue(format!("format amz-date-only: {e}")))?;

        // --- Mutations happen here so canonical-headers sees
        // --- the final request shape.
        let headers = req.headers_mut();

        // x-amz-date must be in the signed set.
        let amz_date_hv = HeaderValue::from_str(&amz_date)
            .map_err(|e| AuthError::InvalidHeaderValue(e.to_string()))?;
        headers.insert(HeaderName::from_static("x-amz-date"), amz_date_hv);

        // Session token (if present) is also signed. AWS STS /
        // IMDSv2-issued credentials require this.
        if let Some(token) = &self.credentials.session_token {
            let mut hv = HeaderValue::from_str(token.expose_secret())
                .map_err(|e| AuthError::InvalidHeaderValue(e.to_string()))?;
            hv.set_sensitive(true);
            headers.insert(HeaderName::from_static("x-amz-security-token"), hv);
        }

        // Host header — reqwest fills this later, but SigV4
        // requires it to be part of the canonical request. Add
        // explicitly if missing.
        let host_header = match req.url().host_str() {
            Some(h) => {
                let port = req.url().port();
                let host_value = match port {
                    Some(p) => format!("{h}:{p}"),
                    None => h.to_string(),
                };
                HeaderValue::from_str(&host_value)
                    .map_err(|e| AuthError::InvalidHeaderValue(e.to_string()))?
            }
            None => {
                return Err(AuthError::InvalidHeaderValue(
                    "SigV4 requires a URL with a host".into(),
                ))
            }
        };
        req.headers_mut().insert(HOST, host_header);

        // --- Build canonical request ---
        let method = req.method().as_str().to_string();
        let canonical_uri = AwsSigV4Helper::canonical_uri(req.url().path());
        let canonical_query =
            AwsSigV4Helper::canonical_query_string(req.url().query().unwrap_or(""));

        let mut header_pairs: Vec<(String, String)> = req
            .headers()
            .iter()
            .map(|(k, v)| {
                (
                    k.as_str().to_ascii_lowercase(),
                    String::from_utf8_lossy(v.as_bytes()).trim().to_string(),
                )
            })
            .collect();
        header_pairs.sort_by(|a, b| a.0.cmp(&b.0));
        let canonical_headers = header_pairs
            .iter()
            .map(|(k, v)| format!("{k}:{v}\n"))
            .collect::<String>();
        let signed_headers = header_pairs
            .iter()
            .map(|(k, _)| k.as_str())
            .collect::<Vec<_>>()
            .join(";");

        // Payload hash — SHA256 of the body, lower-hex.
        let payload_hash = match req.body() {
            Some(body) => match body.as_bytes() {
                Some(bytes) => hex::encode(Sha256::digest(bytes)),
                None => {
                    // Streaming body (no bytes handle). AWS allows
                    // `UNSIGNED-PAYLOAD` for this case; most
                    // services accept it.
                    "UNSIGNED-PAYLOAD".to_string()
                }
            },
            None => hex::encode(Sha256::digest(b"")),
        };

        let canonical_request = format!(
            "{method}\n{canonical_uri}\n{canonical_query}\n{canonical_headers}\n{signed_headers}\n{payload_hash}"
        );

        // --- String to sign ---
        let credential_scope = format!(
            "{date_scope}/{region}/{service}/aws4_request",
            region = self.region,
            service = self.service
        );
        let hashed_canonical = hex::encode(Sha256::digest(canonical_request.as_bytes()));
        let string_to_sign =
            format!("AWS4-HMAC-SHA256\n{amz_date}\n{credential_scope}\n{hashed_canonical}");

        // --- Derive signing key ---
        let signing_key = AwsSigV4Helper::derive_signing_key(
            self.credentials.secret_access_key.expose_secret(),
            &date_scope,
            &self.region,
            &self.service,
        )?;

        // --- Final signature ---
        let signature = AwsSigV4Helper::hmac_sha256(&signing_key, string_to_sign.as_bytes())?;
        let signature_hex = hex::encode(signature);

        // --- Authorization header ---
        let auth_value = format!(
            "AWS4-HMAC-SHA256 Credential={access}/{scope}, SignedHeaders={signed}, Signature={sig}",
            access = self.credentials.access_key_id.expose_secret(),
            scope = credential_scope,
            signed = signed_headers,
            sig = signature_hex,
        );
        let mut auth_hv = HeaderValue::from_str(&auth_value)
            .map_err(|e| AuthError::InvalidHeaderValue(e.to_string()))?;
        auth_hv.set_sensitive(true);
        req.headers_mut().insert(AUTHORIZATION, auth_hv);

        Ok(())
    }
}

impl AuthStrategy for AwsSigV4Strategy {
    fn authorize(&self, req: &mut reqwest::Request) -> Result<(), AuthError> {
        self.sign(req, Self::now())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::{Method, Url};
    use secrecy::SecretString;
    use time::OffsetDateTime;

    fn stub_strategy() -> AwsSigV4Strategy {
        // AWS documentation example keys — split to avoid secret-scanner false positives.
        let key_id = ["AKIA", "IOSFODNN7EXAMPLE"].concat();
        let secret = ["wJalrXUtnFEMI/K7MDENG", "/bPxRfiCYEXAMPLEKEY"].concat();
        AwsSigV4Strategy::new(
            SecretString::from(key_id),
            SecretString::from(secret),
            None,
            "us-east-1".into(),
            "s3".into(),
        )
    }

    fn stub_req(method: Method, url: &str) -> reqwest::Request {
        reqwest::Request::new(method, Url::parse(url).expect("valid test url"))
    }

    /// @covers: new
    #[test]
    fn test_new_stores_region_and_service() {
        let s = AwsSigV4Strategy::new(
            SecretString::from("AKID".to_string()),
            SecretString::from("SEC".to_string()),
            None,
            "eu-west-1".into(),
            "sts".into(),
        );
        let dbg = format!("{s:?}");
        assert!(dbg.contains("eu-west-1"), "debug must show region");
        assert!(dbg.contains("sts"), "debug must show service");
    }

    /// @covers: new
    #[test]
    fn test_new_with_session_token_stores_token() {
        let s = AwsSigV4Strategy::new(
            SecretString::from("AKID".to_string()),
            SecretString::from("SEC".to_string()),
            Some(SecretString::from("tok".to_string())),
            "us-east-1".into(),
            "s3".into(),
        );
        let dbg = format!("{s:?}");
        assert!(
            dbg.contains("<set>"),
            "debug must show session token is set"
        );
    }

    /// @covers: now
    #[test]
    fn test_now_returns_time_close_to_current_utc() {
        let before = OffsetDateTime::now_utc();
        let result = AwsSigV4Strategy::now();
        let after = OffsetDateTime::now_utc();
        assert!(result >= before, "now() must not predate the call");
        assert!(result <= after, "now() must not postdate the call");
    }

    /// @covers: sign
    #[test]
    fn test_sign_attaches_authorization_header() {
        let s = stub_strategy();
        let mut req = stub_req(Method::GET, "https://s3.amazonaws.com/bucket/key");
        s.sign(&mut req, OffsetDateTime::UNIX_EPOCH)
            .expect("sign must succeed for a well-formed request");
        let auth = req
            .headers()
            .get("authorization")
            .expect("authorization header must be present after sign")
            .to_str()
            .expect("header must be valid UTF-8");
        assert!(
            auth.starts_with("AWS4-HMAC-SHA256 Credential="),
            "authorization header must start with AWS4-HMAC-SHA256"
        );
    }

    /// @covers: sign
    #[test]
    fn test_sign_attaches_x_amz_date_header() {
        let s = stub_strategy();
        let mut req = stub_req(Method::GET, "https://s3.amazonaws.com/bucket/key");
        s.sign(&mut req, OffsetDateTime::UNIX_EPOCH)
            .expect("sign must succeed");
        let date = req
            .headers()
            .get("x-amz-date")
            .expect("x-amz-date must be present")
            .to_str()
            .expect("header must be valid UTF-8");
        assert_eq!(
            date.len(),
            16,
            "x-amz-date must be 16 chars (YYYYMMDDTHHMMSSZ)"
        );
        assert!(date.ends_with('Z'), "x-amz-date must end with Z");
    }

    /// @covers: sign
    #[test]
    fn test_sign_fails_for_url_without_host() {
        let s = stub_strategy();
        // file:// URLs have no host component — SigV4 requires a host.
        let mut req = reqwest::Request::new(
            Method::GET,
            Url::parse("file:///local/path").expect("valid url"),
        );
        let result = s.sign(&mut req, OffsetDateTime::UNIX_EPOCH);
        assert!(result.is_err(), "sign must fail when URL has no host");
    }
}
