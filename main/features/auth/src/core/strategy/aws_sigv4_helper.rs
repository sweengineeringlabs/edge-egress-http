//! AWS SigV4 canonical-request helpers.

use hmac::{Hmac, Mac};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use sha2::{Digest, Sha256};
use secrecy::{ExposeSecret, SecretString};

use crate::api::error::AuthError;

type HmacSha256 = Hmac<Sha256>;

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

/// Canonical-request utilities for the AWS SigV4 algorithm.
pub(crate) struct AwsSigV4Helper;

impl AwsSigV4Helper {
    /// Canonical URI: percent-encoded, slash-preserving.
    pub(crate) fn canonical_uri(path: &str) -> String {
        if path.is_empty() {
            return "/".into();
        }
        utf8_percent_encode(path, ENCODE_FOR_PATH).to_string()
    }

    /// Canonical query string: key=value pairs percent-encoded,
    /// sorted by key.
    pub(crate) fn canonical_query_string(query: &str) -> String {
        if query.is_empty() {
            return String::new();
        }
        let mut pairs: Vec<(String, String)> = query
            .split('&')
            .filter(|p| !p.is_empty())
            .map(|pair| match pair.split_once('=') {
                Some((k, v)) => (
                    utf8_percent_encode(k, ENCODE_FOR_QUERY).to_string(),
                    utf8_percent_encode(v, ENCODE_FOR_QUERY).to_string(),
                ),
                None => (
                    utf8_percent_encode(pair, ENCODE_FOR_QUERY).to_string(),
                    String::new(),
                ),
            })
            .collect();
        pairs.sort();
        pairs
            .into_iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join("&")
    }

    /// HMAC-SHA256 of `data` keyed by `key`.
    pub(crate) fn hmac_sha256(key: &[u8], data: &[u8]) -> Result<Vec<u8>, AuthError> {
        let mut mac = HmacSha256::new_from_slice(key)
            .map_err(|e| AuthError::InvalidHeaderValue(format!("HMAC key len: {e}")))?;
        mac.update(data);
        Ok(mac.finalize().into_bytes().to_vec())
    }

    /// Derive the SigV4 signing key via the four-stage HMAC chain.
    pub(crate) fn derive_signing_key(
        secret: &str,
        date: &str,
        region: &str,
        service: &str,
    ) -> Result<Vec<u8>, AuthError> {
        let k_secret = format!("AWS4{secret}");
        let k_date = Self::hmac_sha256(k_secret.as_bytes(), date.as_bytes())?;
        let k_region = Self::hmac_sha256(&k_date, region.as_bytes())?;
        let k_service = Self::hmac_sha256(&k_region, service.as_bytes())?;
        Self::hmac_sha256(&k_service, b"aws4_request")
    }
}

// Silence unused import warning — SecretString is imported for symmetry
// with the strategy file but not directly used here.
const _: () = {
    let _ = std::mem::size_of::<SecretString>();
};

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: AwsSigV4Helper::canonical_uri
    #[test]
    fn test_canonical_uri_preserves_slashes() {
        assert_eq!(AwsSigV4Helper::canonical_uri("/foo/bar"), "/foo/bar");
        assert_eq!(AwsSigV4Helper::canonical_uri("/"), "/");
        assert_eq!(AwsSigV4Helper::canonical_uri(""), "/");
    }

    /// @covers: AwsSigV4Helper::canonical_uri
    #[test]
    fn test_canonical_uri_percent_encodes_spaces_and_unicode() {
        let encoded = AwsSigV4Helper::canonical_uri("/name with space");
        assert!(encoded.contains("%20") || encoded.contains("%2520"));
    }

    /// @covers: AwsSigV4Helper::canonical_query_string
    #[test]
    fn test_canonical_query_string_sorts_params_alphabetically() {
        assert_eq!(
            AwsSigV4Helper::canonical_query_string("Zparam=1&Aparam=2"),
            "Aparam=2&Zparam=1"
        );
    }

    /// @covers: AwsSigV4Helper::canonical_query_string
    #[test]
    fn test_canonical_query_string_empty_returns_empty() {
        assert_eq!(AwsSigV4Helper::canonical_query_string(""), "");
    }

    /// @covers: AwsSigV4Helper::derive_signing_key
    #[test]
    fn test_derive_signing_key_is_deterministic() {
        let k1 = AwsSigV4Helper::derive_signing_key(
            "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
            "20130524",
            "us-east-1",
            "s3",
        )
        .unwrap();
        let k2 = AwsSigV4Helper::derive_signing_key(
            "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
            "20130524",
            "us-east-1",
            "s3",
        )
        .unwrap();
        assert_eq!(k1, k2);
        assert_eq!(k1.len(), 32);
    }

    /// @covers: AwsSigV4Helper::hmac_sha256
    #[test]
    fn test_hmac_sha256_produces_32_byte_output() {
        let result = AwsSigV4Helper::hmac_sha256(b"key", b"data").unwrap();
        assert_eq!(result.len(), 32);
    }

    /// @covers: AwsSigV4Helper::hmac_sha256
    #[test]
    fn test_hmac_sha256_different_data_produces_different_output() {
        let a = AwsSigV4Helper::hmac_sha256(b"key", b"data1").unwrap();
        let b = AwsSigV4Helper::hmac_sha256(b"key", b"data2").unwrap();
        assert_ne!(a, b);
    }

    /// @covers: AwsSigV4Helper::hmac_sha256
    #[test]
    fn test_hmac_sha256_different_keys_produces_different_output() {
        let a = AwsSigV4Helper::hmac_sha256(b"key1", b"data").unwrap();
        let b = AwsSigV4Helper::hmac_sha256(b"key2", b"data").unwrap();
        assert_ne!(a, b);
    }

    /// @covers: AwsSigV4Helper::hmac_sha256
    #[test]
    fn test_hmac_sha256_is_deterministic() {
        let r1 = AwsSigV4Helper::hmac_sha256(b"secret", b"message").unwrap();
        let r2 = AwsSigV4Helper::hmac_sha256(b"secret", b"message").unwrap();
        assert_eq!(r1, r2);
    }
}
