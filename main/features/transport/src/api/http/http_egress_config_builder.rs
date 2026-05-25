//! Fluent builder for [`HttpEgressConfig`].

use super::http_egress_config::HttpEgressConfig;
use crate::api::value_object::HttpConfig;

/// Fluent builder for [`HttpEgressConfig`].
///
/// Construct via [`HttpEgressConfigBuilder::new`] and chain setter methods,
/// then call [`build`](Self::build) to obtain the final [`HttpEgressConfig`].
#[allow(dead_code)]
pub struct HttpEgressConfigBuilder {
    http: HttpConfig,
    cassette_name: String,
}

impl HttpEgressConfigBuilder {
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

    /// Consume the builder and return the configured [`HttpEgressConfig`].
    ///
    /// Uses SWE defaults for all middleware layers.
    pub fn build(self) -> HttpEgressConfig {
        HttpEgressConfig {
            http: self.http,
            auth: swe_edge_egress_auth::AuthConfig::None,
            token_source: None,
            retry: swe_edge_egress_retry::RetryConfig::default(),
            rate: swe_edge_egress_rate::RateConfig::default(),
            breaker: swe_edge_egress_breaker::BreakerConfig::default(),
            cache: swe_edge_egress_cache::CacheConfig::default(),
            cassette: swe_edge_egress_cassette::CassetteConfig::disabled(),
            cassette_name: self.cassette_name,
            tls: swe_edge_egress_tls::TlsConfig::None,
        }
    }
}

impl Default for HttpEgressConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_builder_with_defaults() {
        let _cfg = HttpEgressConfigBuilder::new().build();
    }

    /// @covers: with_http
    #[test]
    fn test_with_http_sets_base_url() {
        let cfg = HttpEgressConfigBuilder::new()
            .with_http(HttpConfig::with_base_url("https://api.example.com"))
            .build();
        assert_eq!(
            cfg.http.base_url.as_deref(),
            Some("https://api.example.com")
        );
    }

    /// @covers: with_cassette_name
    #[test]
    fn test_with_cassette_name_sets_name() {
        let cfg = HttpEgressConfigBuilder::new()
            .with_cassette_name("my-fixture")
            .build();
        assert_eq!(cfg.cassette_name, "my-fixture");
    }

    /// @covers: build
    #[test]
    fn test_build_returns_config_with_defaults() {
        let cfg = HttpEgressConfigBuilder::new().build();
        assert!(cfg.token_source.is_none());
    }
}
