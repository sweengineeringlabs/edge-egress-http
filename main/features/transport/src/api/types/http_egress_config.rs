//! Aggregate configuration for assembling a fully-wired HttpEgress.

use std::sync::Arc;

use crate::api::types::http::http_config::HttpConfig;

/// Aggregate middleware config for assembling a [`DefaultHttpEgress`].
#[derive(Debug, Clone)]
pub struct HttpEgressConfig {
    pub http: HttpConfig,
    /// Static auth strategy (Bearer/Basic/Header/Digest/AwsSigV4).
    /// Ignored when `token_source` is `Some` — OAuth takes precedence.
    pub auth: swe_edge_egress_auth::AuthConfig,
    /// OAuth token source. When set, replaces the static `auth` layer.
    pub token_source: Option<Arc<dyn swe_edge_egress_oauth::OAuthTokenSource>>,
    pub retry: swe_edge_egress_retry::RetryConfig,
    pub rate: swe_edge_egress_rate::RateConfig,
    pub breaker: swe_edge_egress_breaker::BreakerConfig,
    pub cache: swe_edge_egress_cache::CacheConfig,
    pub cassette: swe_edge_egress_cassette::CassetteConfig,
    /// On-disk cassette fixture name (no extension).
    pub cassette_name: String,
    pub tls: swe_edge_egress_tls::TlsConfig,
}
