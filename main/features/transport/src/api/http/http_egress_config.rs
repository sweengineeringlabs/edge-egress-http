//! Aggregate configuration for assembling a fully-wired [`HttpEgress`].

use std::sync::Arc;

use crate::api::value_object::HttpConfig;

/// Aggregate middleware config for assembling a [`DefaultHttpEgress`].
#[derive(Debug, Clone)]
pub struct HttpEgressConfig {
    pub http: HttpConfig,
    /// Static auth strategy (Bearer/Basic/Header/Digest/AwsSigV4).
    /// Ignored when `token_source` is `Some` — OAuth takes precedence.
    pub auth: swe_edge_egress_auth::AuthConfig,
    /// OAuth token source. When set, replaces the static `auth` layer.
    /// Provide an `Arc<dyn OAuthTokenSource>` from your implementation crate.
    pub token_source: Option<Arc<dyn swe_edge_egress_oauth::OAuthTokenSource>>,
    pub retry: swe_edge_egress_retry::RetryConfig,
    pub rate: swe_edge_egress_rate::RateConfig,
    pub breaker: swe_edge_egress_breaker::BreakerConfig,
    pub cache: swe_edge_egress_cache::CacheConfig,
    pub cassette: swe_edge_egress_cassette::CassetteConfig,
    /// On-disk cassette fixture name (no extension). Maps to
    /// `<cassette_dir>/<cassette_name>.yaml`.
    pub cassette_name: String,
    pub tls: swe_edge_egress_tls::TlsConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_config(base_url: &str, cassette_name: &str) -> HttpEgressConfig {
        HttpEgressConfig {
            http: HttpConfig::with_base_url(base_url),
            auth: swe_edge_egress_auth::AuthConfig::None,
            token_source: None,
            retry: swe_edge_egress_retry::RetryConfig::default(),
            rate: swe_edge_egress_rate::RateConfig::default(),
            breaker: swe_edge_egress_breaker::BreakerConfig::default(),
            cache: swe_edge_egress_cache::CacheConfig::default(),
            cassette: swe_edge_egress_cassette::CassetteConfig::disabled(),
            cassette_name: cassette_name.to_string(),
            tls: swe_edge_egress_tls::TlsConfig::None,
        }
    }

    #[test]
    fn test_http_egress_config_stores_http_config_base_url() {
        let cfg = make_config("https://api.example.com", "test");
        assert_eq!(
            cfg.http.base_url.as_deref(),
            Some("https://api.example.com")
        );
        assert_eq!(cfg.cassette_name, "test");
    }

    #[test]
    fn test_http_egress_config_cassette_name_is_independent_on_clone() {
        let cfg = make_config("https://api.example.com", "original");
        let mut cloned = cfg.clone();
        cloned.cassette_name = "cloned".to_string();
        assert_eq!(cfg.cassette_name, "original");
        assert_eq!(cloned.cassette_name, "cloned");
    }
}
