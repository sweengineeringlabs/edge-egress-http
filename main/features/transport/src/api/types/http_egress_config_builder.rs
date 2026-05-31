//! Fluent builder for [`HttpEgressConfig`].

use crate::api::types::http::http_config::HttpConfig;
use crate::api::types::http_egress_config::HttpEgressConfig;

/// Fluent builder for [`HttpEgressConfig`].
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
