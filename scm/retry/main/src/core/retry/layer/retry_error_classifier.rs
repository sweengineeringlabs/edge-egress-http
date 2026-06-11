//! Error classification for the retry middleware.

/// Classifies reqwest-middleware errors as transient (retry-worthy)
/// or non-transient.
pub(crate) struct RetryErrorClassifier;

impl RetryErrorClassifier {
    /// Classify a `reqwest_middleware::Error` as transient
    /// (retry-worthy) or non-transient. Transient failures:
    /// connection reset, DNS temporary failures, read timeout, etc.
    /// Non-transient: certificate errors, malformed URL.
    pub(crate) fn is_transient(err: &reqwest_middleware::Error) -> bool {
        match err {
            reqwest_middleware::Error::Reqwest(inner) => {
                // Most reqwest errors stem from I/O and are worth
                // retrying. Certificate / redirect / builder errors
                // are not. `is_connect` catches connect-time issues
                // (good to retry); `is_timeout` is transient by
                // definition; everything else we treat as transient
                // unless clearly a config issue.
                inner.is_timeout() || inner.is_connect() || inner.is_request()
            }
            // Middleware-level errors (e.g. from our auth layer)
            // are generally NOT transient — they reflect
            // configuration or credential problems. Don't retry.
            reqwest_middleware::Error::Middleware(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: is_transient
    #[test]
    fn test_is_transient_middleware_error_is_not_transient() {
        let err = reqwest_middleware::Error::middleware(std::io::Error::other("config error"));
        assert!(
            !RetryErrorClassifier::is_transient(&err),
            "Middleware-level errors must NOT be retried"
        );
    }
}
