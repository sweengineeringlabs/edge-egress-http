//! Auth policy schema. Values live in `config/application.toml`.
//!
//! **Credential resolution is env-var-only.** The config stores
//! the NAME of the env var holding the credential, not the
//! credential itself. This is enforced at the schema level —
//! there are no `token: String` or `password: String` fields on
//! any variant.

use serde::Deserialize;

use crate::api::error::AuthError;

/// Auth policy schema. A tagged enum on the `kind` field so
/// config like `kind = "bearer"; token_env = "..."` deserializes
/// into the right variant.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum AuthConfig {
    /// Pass-through — middleware attached but attaches no
    /// credential. The baseline; consumers must explicitly
    /// switch to a real scheme to activate auth.
    #[default]
    None,

    /// `Authorization: Bearer <token>`. Token is read from the
    /// named env var at config-load time.
    Bearer {
        /// Name of the env var holding the bearer token.
        token_env: String,
    },

    /// `Authorization: Basic base64(user:pass)`. User + password
    /// are read from the named env vars at config-load time.
    Basic {
        /// Env var holding the username.
        user_env: String,
        /// Env var holding the password.
        pass_env: String,
    },

    /// Custom credential header (e.g. `x-api-key`,
    /// `x-goog-api-key`). For APIs that use non-standard auth
    /// headers instead of `Authorization`.
    Header {
        /// The HTTP header name to set (lowercase recommended
        /// for clarity; case-insensitive on the wire).
        name: String,
        /// Env var holding the credential value.
        value_env: String,
    },

    /// AWS Signature Version 4 — signs each request with
    /// HMAC-SHA256 using the access-key / secret-key pair
    /// derived from env vars. Suitable for AWS service APIs and
    /// SigV4-compatible endpoints (S3-compatible stores like
    /// MinIO, R2, Ceph RGW).
    AwsSigV4 {
        /// Env var holding the AWS access key ID.
        access_key_env: String,
        /// Env var holding the AWS secret access key.
        secret_key_env: String,
        /// Env var holding the session token for temporary
        /// credentials (STS, IMDSv2). Optional — omit for
        /// long-term credentials.
        #[serde(default)]
        session_token_env: Option<String>,
        /// AWS region (e.g. `"us-east-1"`).
        region: String,
        /// AWS service name (e.g. `"s3"`, `"sts"`,
        /// `"execute-api"`).
        service: String,
    },
}

impl swe_edge_configbuilder::ConfigSection for AuthConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies
        "auth"
    }
}

impl AuthConfig {
    /// Parse from TOML text.
    pub fn from_config(toml_text: &str) -> Result<Self, AuthError> {
        toml::from_str(toml_text).map_err(|e| AuthError::ParseFailed(e.to_string()))
    }
}
