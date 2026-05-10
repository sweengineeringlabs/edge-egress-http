use std::sync::Arc;
use std::time::Duration;

use reqwest_middleware::{ClientBuilder, Middleware};
use swe_edge_egress_tls::TlsApplier;
use swe_observ_metrics::MetricsProvider;

use crate::core::{DefaultHttpOutbound, MetricsHttpOutbound};

pub use crate::api::port::{HttpOutbound, HttpOutboundError, HttpOutboundResult};
pub use crate::api::value_object::{
    FormPart, HttpAuth, HttpBody, HttpConfig, HttpMethod, HttpRequest, HttpResponse,
    HttpStreamResponse,
};

/// Aggregate middleware config for assembling a [`DefaultHttpOutbound`].
#[derive(Debug, Clone)]
pub struct HttpOutboundConfig {
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

/// Build a fully assembled [`HttpOutbound`] from the supplied config.
///
/// Assembly order: TLS → reqwest client → auth/oauth → retry → rate →
/// breaker → cache → cassette.
///
/// When `config.oauth` is `Some`, the OAuth token-refresh layer replaces the
/// static `config.auth` layer. Both cannot be active simultaneously.
pub fn http_outbound(
    config: HttpOutboundConfig,
) -> Result<impl HttpOutbound, HttpOutboundBuildError> {
    let retry = swe_edge_egress_retry::Builder::with_config(config.retry).build()?;
    let rate = swe_edge_egress_rate::Builder::with_config(config.rate).build()?;
    let breaker = swe_edge_egress_breaker::Builder::with_config(config.breaker).build()?;
    let cache = swe_edge_egress_cache::Builder::with_config(config.cache).build()?;
    let cassette = swe_edge_egress_cassette::Builder::with_config(config.cassette)
        .build(&config.cassette_name)?;
    let tls = swe_edge_egress_tls::Builder::with_config(config.tls).build()?;

    if let Some(source) = config.token_source {
        assemble(
            config.http,
            swe_edge_egress_oauth::builder()
                .with_token_source(source)
                .build()
                .expect("token_source was Some so build cannot fail"),
            retry,
            rate,
            breaker,
            cache,
            cassette,
            tls,
        )
    } else {
        assemble(
            config.http,
            swe_edge_egress_auth::Builder::with_config(config.auth).build()?,
            retry,
            rate,
            breaker,
            cache,
            cassette,
            tls,
        )
    }
}

/// Build an [`HttpOutbound`] with OAuth token-refresh auth and SWE defaults
/// for every other middleware layer.
///
/// Shorthand for `http_outbound` when the caller supplies an
/// [`OAuthTokenSource`] and accepts the SWE defaults for retry, rate,
/// breaker, cache, cassette, and TLS.
pub fn http_outbound_oauth(
    http: HttpConfig,
    source: Arc<dyn swe_edge_egress_oauth::OAuthTokenSource>,
) -> Result<impl HttpOutbound, HttpOutboundBuildError> {
    assemble(
        http,
        swe_edge_egress_oauth::builder()
            .with_token_source(source)
            .build()
            .expect("token_source is Some — build cannot fail"),
        swe_edge_egress_retry::builder()?.build()?,
        swe_edge_egress_rate::builder()?.build()?,
        swe_edge_egress_breaker::builder()?.build()?,
        swe_edge_egress_cache::builder()?.build()?,
        // Cassette is disabled in production convenience functions — it is
        // test infrastructure and must not intercept real outbound calls.
        swe_edge_egress_cassette::Builder::with_config(
            swe_edge_egress_cassette::CassetteConfig::disabled(),
        )
        .build("unused")?,
        swe_edge_egress_tls::builder()?.build()?,
    )
}

/// Build an [`HttpOutbound`] with a static [`AuthConfig`] and SWE defaults
/// for every other middleware layer.
///
/// Shorthand for `http_outbound` when the caller uses an env-var-backed
/// credential (Bearer, Header, Basic, etc.) and accepts the SWE defaults.
pub fn http_outbound_with_auth(
    http: HttpConfig,
    auth: swe_edge_egress_auth::AuthConfig,
) -> Result<impl HttpOutbound, HttpOutboundBuildError> {
    assemble(
        http,
        swe_edge_egress_auth::Builder::with_config(auth).build()?,
        swe_edge_egress_retry::builder()?.build()?,
        swe_edge_egress_rate::builder()?.build()?,
        swe_edge_egress_breaker::builder()?.build()?,
        swe_edge_egress_cache::builder()?.build()?,
        // Cassette disabled — production convenience function.
        swe_edge_egress_cassette::Builder::with_config(
            swe_edge_egress_cassette::CassetteConfig::disabled(),
        )
        .build("unused")?,
        swe_edge_egress_tls::builder()?.build()?,
    )
}

/// Build an [`HttpOutbound`] using the SWE-shipped defaults for every
/// middleware layer (pass-through auth, no TLS, sensible retry/rate/breaker
/// policies from each crate's `config/application.toml`).
pub fn default_http_outbound() -> Result<impl HttpOutbound, HttpOutboundBuildError> {
    assemble(
        HttpConfig::default(),
        swe_edge_egress_auth::builder()?.build()?,
        swe_edge_egress_retry::builder()?.build()?,
        swe_edge_egress_rate::builder()?.build()?,
        swe_edge_egress_breaker::builder()?.build()?,
        swe_edge_egress_cache::builder()?.build()?,
        swe_edge_egress_cassette::builder()?.build("default")?,
        swe_edge_egress_tls::builder()?.build()?,
    )
}

/// Build a fully-assembled [`HttpOutbound`] using the provided [`HttpConfig`]
/// and SWE defaults for every middleware layer.
///
/// Same middleware stack as [`default_http_outbound`] but with caller-supplied
/// transport settings (base URL, timeouts, headers, etc.).
pub fn default_http_outbound_with_config(
    http: HttpConfig,
) -> Result<impl HttpOutbound, HttpOutboundBuildError> {
    assemble(
        http,
        swe_edge_egress_auth::builder()?.build()?,
        swe_edge_egress_retry::builder()?.build()?,
        swe_edge_egress_rate::builder()?.build()?,
        swe_edge_egress_breaker::builder()?.build()?,
        swe_edge_egress_cache::builder()?.build()?,
        swe_edge_egress_cassette::Builder::with_config(
            swe_edge_egress_cassette::CassetteConfig::disabled(),
        )
        .build("unused")?,
        swe_edge_egress_tls::builder()?.build()?,
    )
}

/// Wrap any [`HttpOutbound`] with per-call metrics observation.
///
/// Consumers call this after any of the factory functions to add observability
/// without changing how the outbound is configured:
///
/// ```rust,ignore
/// let outbound = observe_http_outbound(default_http_outbound()?, metrics_provider);
/// ```
pub fn observe_http_outbound(
    inner: impl HttpOutbound + 'static,
    provider: Arc<dyn MetricsProvider>,
) -> impl HttpOutbound {
    MetricsHttpOutbound::new(Arc::new(inner), provider)
}

/// Build a minimal [`HttpOutbound`] from just an [`HttpConfig`] — no middleware layers.
///
/// All `HttpConfig` fields are honoured: `timeout_secs`, `connect_timeout_secs`,
/// `user_agent`, `follow_redirects`, `max_redirects`, `default_headers`, and
/// `max_response_bytes`.  Useful for integration tests and simple deployments
/// that do not need the full auth/retry/rate/breaker/cache/cassette stack.
pub fn plain_http_outbound(
    config: HttpConfig,
) -> Result<impl HttpOutbound, HttpOutboundBuildError> {
    let mut cb = reqwest::Client::builder();
    cb = cb.timeout(Duration::from_secs(config.timeout_secs));
    cb = cb.connect_timeout(Duration::from_secs(config.connect_timeout_secs));
    if let Some(ua) = &config.user_agent {
        cb = cb.user_agent(ua);
    }
    if config.follow_redirects {
        cb = cb.redirect(reqwest::redirect::Policy::limited(
            config.max_redirects as usize,
        ));
    } else {
        cb = cb.redirect(reqwest::redirect::Policy::none());
    }
    if !config.default_headers.is_empty() {
        let mut map = reqwest::header::HeaderMap::new();
        for (k, v) in &config.default_headers {
            if let (Ok(name), Ok(val)) = (
                reqwest::header::HeaderName::from_bytes(k.as_bytes()),
                reqwest::header::HeaderValue::from_str(v),
            ) {
                map.insert(name, val);
            }
        }
        cb = cb.default_headers(map);
    }
    let client = reqwest_middleware::ClientBuilder::new(cb.build()?).build();
    Ok(DefaultHttpOutbound::new(
        client,
        config.base_url,
        config.max_response_bytes,
    ))
}

#[allow(clippy::too_many_arguments)]
fn assemble<A: Middleware>(
    http_cfg: HttpConfig,
    auth: A,
    retry: swe_edge_egress_retry::RetryLayer,
    rate: swe_edge_egress_rate::RateLayer,
    breaker: swe_edge_egress_breaker::BreakerLayer,
    cache: swe_edge_egress_cache::CacheLayer,
    cassette: swe_edge_egress_cassette::CassetteLayer,
    tls: swe_edge_egress_tls::TlsLayer,
) -> Result<DefaultHttpOutbound, HttpOutboundBuildError> {
    let mut cb = reqwest::Client::builder();
    cb = tls.apply_to(cb)?;
    cb = cb.timeout(Duration::from_secs(http_cfg.timeout_secs));
    cb = cb.connect_timeout(Duration::from_secs(http_cfg.connect_timeout_secs));
    if let Some(ua) = &http_cfg.user_agent {
        cb = cb.user_agent(ua);
    }
    if http_cfg.follow_redirects {
        cb = cb.redirect(reqwest::redirect::Policy::limited(
            http_cfg.max_redirects as usize,
        ));
    } else {
        cb = cb.redirect(reqwest::redirect::Policy::none());
    }
    if !http_cfg.default_headers.is_empty() {
        let mut map = reqwest::header::HeaderMap::new();
        for (k, v) in &http_cfg.default_headers {
            if let (Ok(name), Ok(val)) = (
                reqwest::header::HeaderName::from_bytes(k.as_bytes()),
                reqwest::header::HeaderValue::from_str(v),
            ) {
                map.insert(name, val);
            }
        }
        cb = cb.default_headers(map);
    }
    let reqwest_client = cb.build()?;

    let client = ClientBuilder::new(reqwest_client)
        .with(auth)
        .with(retry)
        .with(rate)
        .with(breaker)
        .with(cache)
        .with(cassette)
        .build();

    Ok(DefaultHttpOutbound::new(
        client,
        http_cfg.base_url,
        http_cfg.max_response_bytes,
    ))
}
