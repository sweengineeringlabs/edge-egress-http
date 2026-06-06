//! Error type for assembling an [`HttpEgress`] at startup.

/// Error returned when assembling an [`HttpEgress`] fails at startup.
#[derive(Debug, thiserror::Error)]
pub enum HttpEgressBuildError {
    /// Auth middleware assembly failed.
    #[cfg(feature = "auth")]
    #[error("auth: {0}")]
    Auth(#[from] swe_edge_egress_auth::AuthError),
    /// Retry middleware assembly failed.
    #[cfg(feature = "retry")]
    #[error("retry: {0}")]
    Retry(#[from] swe_edge_egress_retry::RetryError),
    /// Rate-limiting middleware assembly failed.
    #[cfg(feature = "rate")]
    #[error("rate: {0}")]
    Rate(#[from] swe_edge_egress_rate::RateError),
    /// Circuit-breaker middleware assembly failed.
    #[cfg(feature = "breaker")]
    #[error("breaker: {0}")]
    Breaker(#[from] swe_edge_egress_breaker::BreakerError),
    /// Cache middleware assembly failed.
    #[cfg(feature = "cache")]
    #[error("cache: {0}")]
    Cache(#[from] swe_edge_egress_cache::CacheError),
    /// Cassette middleware assembly failed.
    #[cfg(feature = "cassette")]
    #[error("cassette: {0}")]
    Cassette(#[from] swe_edge_egress_cassette::CassetteError),
    /// TLS middleware assembly failed.
    #[cfg(feature = "tls")]
    #[error("tls: {0}")]
    Tls(#[from] swe_edge_egress_tls::TlsError),
    /// OAuth builder assembly failed.
    #[cfg(feature = "oauth")]
    #[error("oauth: {0}")]
    OAuth(#[from] swe_edge_egress_oauth::OAuthError),
    /// Reqwest client construction failed.
    #[error("reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// Loading or validating an optional `[section]` from config failed.
    #[error("config: {0}")]
    Config(#[from] swe_edge_configbuilder::ConfigError),
}
