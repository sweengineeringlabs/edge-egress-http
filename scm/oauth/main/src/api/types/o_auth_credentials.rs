//! `OAuthCredentials` — in-memory OAuth2 credential value.

/// A set of OAuth2 credentials held in memory.
#[derive(Debug, Clone)]
pub struct OAuthCredentials {
    /// Short-lived bearer access token.
    pub access_token: String,
    /// Long-lived refresh token used to obtain a new access token.
    pub refresh_token: String,
    /// Unix-epoch milliseconds when the access token expires.
    pub expires_at_ms: u64,
    /// OAuth scopes granted to this token.
    pub scopes: Vec<String>,
}
