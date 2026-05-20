//! Transport-level configuration for the default HTTP outbound implementation.
//!
//! [`TransportConfig`] carries the transport-level settings that
//! `DefaultHttpOutbound` (in `core/`) consumes when building a reqwest client.

use crate::api::value_object::HttpConfig;

/// Transport-level configuration for the default (reqwest-backed) HTTP outbound.
///
/// Callers that need only transport-level settings without the full middleware
/// stack use this type to configure the `DefaultHttpOutbound` implementation
/// returned by [`plain_http_outbound`](crate::saf::plain_http_outbound).
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
