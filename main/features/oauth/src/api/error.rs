//! OAuth error types.

/// Errors produced by the OAuth crate.
#[derive(Debug, thiserror::Error)]
pub enum OAuthError {
    /// OAuth credentials could not be located.
    #[error("credentials not found: {0}")]
    CredentialsNotFound(String),
    /// Token refresh request failed.
    #[error("token refresh failed: {0}")]
    RefreshFailed(String),
    /// Underlying HTTP request failed.
    #[error("http error: {0}")]
    Http(String),
    /// Invalid or missing configuration.
    #[error("configuration error: {0}")]
    Configuration(String),
}

/// Convenience alias.
pub type Result<T> = std::result::Result<T, OAuthError>;
