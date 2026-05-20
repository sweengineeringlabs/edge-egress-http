//! Fluent builder for [`HttpOutboundConfig`].

use super::http_outbound_config::HttpOutboundConfig;
use crate::api::value_object::HttpConfig;

/// Fluent builder for [`HttpOutboundConfig`].
///
/// Construct via [`HttpOutboundConfigBuilder::new`] and chain setter methods,
/// then call [`build`](Self::build) to obtain the final [`HttpOutboundConfig`].
#[allow(dead_code)]
pub struct HttpOutboundConfigBuilder {
    http: HttpConfig,
    cassette_name: String,
}

impl HttpOutboundConfigBuilder {
    /// Create a new builder with SWE defaults for all middleware layers.
    pub fn new() -> Self {
        Self {
            http: HttpConfig::default(),
            cassette_name: "default".to_string(),
        }
    }

    /// Set the HTTP transport configuration.
    pub fn with_http(mut self, http: HttpConfig) -> Self {
        self.http = http;
        self
    }

    /// Set the cassette fixture name for recording/replay.
    pub fn with_cassette_name(mut self, name: impl Into<String>) -> Self {
        self.cassette_name = name.into();
        self
    }

    /// Consume the builder and return the configured [`HttpOutboundConfig`].
    ///
    /// Uses SWE defaults for all middleware layers.
    pub fn build(self) -> Result<HttpOutboundConfig, crate::api::http::HttpOutboundBuildError> {
        Ok(HttpOutboundConfig {
            http: self.http,
            auth: swe_edge_egress_auth::AuthConfig::None,
            token_source: None,
            retry: swe_edge_egress_retry::builder()?.config().clone(),
            rate: swe_edge_egress_rate::builder()?.config().clone(),
            breaker: swe_edge_egress_breaker::builder()?.config().clone(),
            cache: swe_edge_egress_cache::builder()?.config().clone(),
            cassette: swe_edge_egress_cassette::CassetteConfig::disabled(),
            cassette_name: self.cassette_name,
            tls: swe_edge_egress_tls::TlsConfig::None,
        })
    }
}

impl Default for HttpOutboundConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_builder_with_defaults() {
        let result = HttpOutboundConfigBuilder::new().build();
        assert!(
            result.is_ok(),
            "default builder must build: {:?}",
            result.err()
        );
    }

    /// @covers: with_http
    #[test]
    fn test_with_http_sets_base_url() {
        let result = HttpOutboundConfigBuilder::new()
            .with_http(HttpConfig::with_base_url("https://api.example.com"))
            .build();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().http.base_url.as_deref(),
            Some("https://api.example.com")
        );
    }

    /// @covers: with_cassette_name
    #[test]
    fn test_with_cassette_name_sets_name() {
        let result = HttpOutboundConfigBuilder::new()
            .with_cassette_name("my-fixture")
            .build();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().cassette_name, "my-fixture");
    }

    /// @covers: build
    #[test]
    fn test_build_returns_config_with_defaults() {
        let cfg = HttpOutboundConfigBuilder::new().build().unwrap();
        assert!(cfg.token_source.is_none());
    }
}
