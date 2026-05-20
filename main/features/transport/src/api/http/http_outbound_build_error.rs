//! Error type for assembling an [`HttpOutbound`] at startup.

/// Error returned when assembling an [`HttpOutbound`] fails at startup.
#[derive(Debug, thiserror::Error)]
pub enum HttpOutboundBuildError {
    #[error("auth: {0}")]
    Auth(#[from] swe_edge_egress_auth::Error),
    #[error("retry: {0}")]
    Retry(#[from] swe_edge_egress_retry::Error),
    #[error("rate: {0}")]
    Rate(#[from] swe_edge_egress_rate::Error),
    #[error("breaker: {0}")]
    Breaker(#[from] swe_edge_egress_breaker::Error),
    #[error("cache: {0}")]
    Cache(#[from] swe_edge_egress_cache::Error),
    #[error("cassette: {0}")]
    Cassette(#[from] swe_edge_egress_cassette::Error),
    #[error("tls: {0}")]
    Tls(#[from] swe_edge_egress_tls::Error),
    #[error("reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_outbound_build_error_display_formats_with_prefix() {
        // The tls::Error::ParseFailed variant is constructible directly.
        let tls_err = swe_edge_egress_tls::Error::ParseFailed("bad config".into());
        let build_err: HttpOutboundBuildError = tls_err.into();
        let msg = build_err.to_string();
        assert!(
            msg.starts_with("tls:"),
            "error message must start with 'tls:', got: {msg:?}"
        );
    }

    #[test]
    fn test_http_outbound_build_error_debug_is_non_empty() {
        let tls_err = swe_edge_egress_tls::Error::ParseFailed("x".into());
        let build_err: HttpOutboundBuildError = tls_err.into();
        let dbg = format!("{:?}", build_err);
        assert!(!dbg.is_empty());
    }
}
