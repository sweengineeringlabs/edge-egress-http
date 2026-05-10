//! OAuth error types.

/// Errors produced by the OAuth crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
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
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_credentials_not_found() {
        let e = Error::CredentialsNotFound("missing".into());
        assert!(e.to_string().contains("missing"));
    }

    #[test]
    fn test_error_display_refresh_failed() {
        let e = Error::RefreshFailed("HTTP 401".into());
        assert!(e.to_string().contains("401"));
    }
}
