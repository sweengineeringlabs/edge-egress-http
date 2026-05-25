//! Transport-level configuration for the default HTTP outbound implementation.

use crate::api::types::http::http_config::HttpConfig;

/// Transport-level configuration for the default (reqwest-backed) HTTP outbound.
pub struct TransportConfig {
    /// Transport-level configuration (timeouts, headers, redirects, etc.).
    pub http: HttpConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_config_stores_http_config() {
        let cfg = TransportConfig {
            http: HttpConfig::with_base_url("https://api.example.com"),
        };
        assert_eq!(
            cfg.http.base_url.as_deref(),
            Some("https://api.example.com")
        );
    }
}
