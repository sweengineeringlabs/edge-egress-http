//! SAF factory functions for assembling [`HttpOutbound`] instances.

use std::sync::Arc;
use std::time::Duration;

use reqwest_middleware::{ClientBuilder, Middleware};
use swe_edge_egress_tls::TlsApplier;
use swe_observ_metrics::MetricsProvider;

use crate::api::http::HttpOutboundBuildError;
use crate::api::http::HttpOutboundConfig;
use crate::api::port::HttpOutbound;
use crate::api::traits::Validator as _;
use crate::api::value_object::HttpConfig;
use crate::core::{DefaultHttpOutbound, MetricsHttpOutbound};

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
    let retry =
        swe_edge_egress_retry::ApplicationConfigBuilder::with_config(config.retry).build()?;
    let rate = swe_edge_egress_rate::ApplicationConfigBuilder::with_config(config.rate).build()?;
    let breaker =
        swe_edge_egress_breaker::ApplicationConfigBuilder::with_config(config.breaker).build()?;
    let cache =
        swe_edge_egress_cache::ApplicationConfigBuilder::with_config(config.cache).build()?;
    let cassette = swe_edge_egress_cassette::ApplicationConfigBuilder::with_config(config.cassette)
        .build(&config.cassette_name)?;
    let tls = swe_edge_egress_tls::ApplicationConfigBuilder::with_config(config.tls).build()?;

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
            swe_edge_egress_auth::ApplicationConfigBuilder::with_config(config.auth).build()?,
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
        swe_edge_egress_cassette::ApplicationConfigBuilder::with_config(
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
        swe_edge_egress_auth::ApplicationConfigBuilder::with_config(auth).build()?,
        swe_edge_egress_retry::builder()?.build()?,
        swe_edge_egress_rate::builder()?.build()?,
        swe_edge_egress_breaker::builder()?.build()?,
        swe_edge_egress_cache::builder()?.build()?,
        // Cassette disabled — production convenience function.
        swe_edge_egress_cassette::ApplicationConfigBuilder::with_config(
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
        swe_edge_egress_cassette::ApplicationConfigBuilder::with_config(
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

/// Validate that an [`HttpConfig`] value is well-formed.
///
/// Returns `Ok(())` when the config is valid, or a human-readable error
/// message explaining what field is invalid and what the expected constraint is.
pub fn validate_http_config(config: &HttpConfig) -> Result<(), String> {
    use crate::core::validator::HttpConfigValidator;
    HttpConfigValidator::new(config).validate()
}

/// Validate any value that implements the [`Validator`] trait.
///
/// This is the generic gateway to the [`Validator`] contract — consumers who
/// implement [`Validator`] on their own types can call this to run validation
/// through the SAF boundary without holding a direct reference to the trait.
pub fn validate<V: crate::api::traits::Validator>(v: &V) -> Result<(), String> {
    v.validate()
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn assemble<A: Middleware>(
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

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: default_http_outbound
    #[test]
    fn test_default_http_outbound_builds_with_swe_defaults() {
        let result = default_http_outbound();
        assert!(
            result.is_ok(),
            "default_http_outbound must build: {:?}",
            result.err()
        );
    }

    /// @covers: plain_http_outbound
    #[test]
    fn test_plain_http_outbound_builds_with_default_config() {
        let result = plain_http_outbound(HttpConfig::default());
        assert!(
            result.is_ok(),
            "plain_http_outbound must build: {:?}",
            result.err()
        );
    }

    /// @covers: validate_http_config
    #[test]
    fn test_validate_http_config_returns_ok_for_defaults() {
        let cfg = HttpConfig::default();
        assert!(validate_http_config(&cfg).is_ok());
    }

    /// @covers: validate
    #[test]
    fn test_validate_delegates_to_validator_trait() {
        struct AlwaysOk;
        impl crate::api::traits::Validator for AlwaysOk {
            fn validate(&self) -> Result<(), String> {
                Ok(())
            }
        }
        assert!(validate(&AlwaysOk).is_ok());
    }

    /// @covers: http_outbound
    #[test]
    fn test_http_outbound_builds_with_none_auth_config() {
        use crate::api::http::{HttpOutboundBuildError, HttpOutboundConfig};
        let retry = swe_edge_egress_retry::builder().unwrap().config().clone();
        let rate = swe_edge_egress_rate::builder().unwrap().config().clone();
        let breaker = swe_edge_egress_breaker::builder().unwrap().config().clone();
        let cache = swe_edge_egress_cache::builder().unwrap().config().clone();
        let cfg = HttpOutboundConfig {
            http: HttpConfig::default(),
            auth: swe_edge_egress_auth::AuthConfig::None,
            token_source: None,
            retry,
            rate,
            breaker,
            cache,
            cassette: swe_edge_egress_cassette::CassetteConfig::disabled(),
            cassette_name: "unused".to_string(),
            tls: swe_edge_egress_tls::TlsConfig::None,
        };
        let result: Result<_, HttpOutboundBuildError> = http_outbound(cfg);
        assert!(
            result.is_ok(),
            "http_outbound must build: {:?}",
            result.err()
        );
    }

    /// @covers: http_outbound_with_auth
    #[test]
    fn test_http_outbound_with_auth_builds_with_none_auth() {
        let result = http_outbound_with_auth(
            HttpConfig::default(),
            swe_edge_egress_auth::AuthConfig::None,
        );
        assert!(
            result.is_ok(),
            "http_outbound_with_auth must build: {:?}",
            result.err()
        );
    }

    /// @covers: default_http_outbound_with_config
    #[test]
    fn test_default_http_outbound_with_config_builds_with_custom_base_url() {
        let cfg = HttpConfig::with_base_url("https://api.example.com");
        let result = default_http_outbound_with_config(cfg);
        assert!(
            result.is_ok(),
            "default_http_outbound_with_config must build: {:?}",
            result.err()
        );
    }

    /// @covers: observe_http_outbound
    #[test]
    fn test_observe_http_outbound_wraps_plain_outbound() {
        use std::sync::Arc;
        use swe_observ_metrics::MetricsProvider;
        let inner = plain_http_outbound(HttpConfig::default()).unwrap();
        let provider: Arc<dyn MetricsProvider> =
            Arc::new(swe_observ_metrics::create_local_metrics_backend());
        let _observed = observe_http_outbound(inner, provider);
    }

    /// @covers: http_outbound_oauth
    #[test]
    fn test_http_outbound_oauth_builds_with_noop_token_source() {
        use std::sync::Arc;
        use swe_edge_egress_oauth::OAuthTokenSource;

        #[derive(Debug)]
        struct NoopTokenSource;
        impl OAuthTokenSource for NoopTokenSource {
            fn get_access_token(
                &self,
            ) -> futures::future::BoxFuture<'_, swe_edge_egress_oauth::Result<String>> {
                Box::pin(async { Ok("noop-token".to_string()) })
            }
        }

        let source: Arc<dyn OAuthTokenSource> = Arc::new(NoopTokenSource);
        let result = http_outbound_oauth(HttpConfig::default(), source);
        assert!(
            result.is_ok(),
            "http_outbound_oauth must build: {:?}",
            result.err()
        );
    }
}
