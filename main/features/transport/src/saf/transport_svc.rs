//! SAF factory methods on [`HttpTransportSvc`] for assembling [`HttpEgress`] instances.

use std::sync::Arc;
use std::time::Duration;
use swe_edge_egress_oauth::OAuthBuilderOps as _;

use reqwest_middleware::{ClientBuilder, Middleware};
use swe_observ_metrics::MetricsProvider;

use crate::api::http::HttpEgressBuildError;
use crate::api::http::HttpEgressConfig;
use crate::api::port::{HttpEgress, HttpStream};
use crate::api::traits::Validator as _;
use crate::api::types::{HttpConfig, HttpTransportSvc};
use crate::core::{DefaultHttpEgress, MetricsHttpEgress};

impl HttpTransportSvc {
    /// Return a config builder pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
    }

    /// Build a fully assembled [`HttpEgress`] from the supplied config.
    ///
    /// Assembly order: TLS → reqwest client → auth/oauth → retry → rate →
    /// breaker → cache → cassette.
    ///
    /// When `config.oauth` is `Some`, the OAuth token-refresh layer replaces the
    /// static `config.auth` layer. Both cannot be active simultaneously.
    pub fn http_egress(
        config: HttpEgressConfig,
    ) -> Result<Box<dyn HttpEgress>, HttpEgressBuildError> {
        let retry = swe_edge_egress_retry::HttpRetrySvc::build_retry_layer(config.retry)?;
        let rate = swe_edge_egress_rate::HttpRateSvc::build_rate_layer(config.rate)?;
        let breaker = swe_edge_egress_breaker::HttpBreakerSvc::build_breaker_layer(config.breaker)?;
        let cache = swe_edge_egress_cache::HttpCacheSvc::build_cache_layer(config.cache)?;
        let cassette = swe_edge_egress_cassette::HttpCassetteSvc::build_cassette_layer(
            config.cassette,
            &config.cassette_name,
        )?;
        let tls = swe_edge_egress_tls::HttpTlsSvc::build_tls_layer(config.tls)?;

        if let Some(source) = config.token_source {
            let oauth = swe_edge_egress_oauth::OAuthSvc::builder()
                .with_token_source(source)
                .build()?;
            Ok(Box::new(Self::assemble(
                config.http,
                oauth,
                retry,
                rate,
                breaker,
                cache,
                cassette,
                tls,
            )?))
        } else {
            Ok(Box::new(Self::assemble(
                config.http,
                swe_edge_egress_auth::AuthSvc::build_auth_middleware(config.auth)?,
                retry,
                rate,
                breaker,
                cache,
                cassette,
                tls,
            )?))
        }
    }

    /// Build an [`HttpEgress`] whose optional middleware are activated by the
    /// consumer's configuration (ADR-006 config-driven activation): a feature is
    /// wired **iff** its `[section]` is present in the loaded config.
    ///
    /// Config-drives `[auth]`, `[tls]`, `[retry]`, `[rate]`, `[breaker]`,
    /// `[cache]`, and `[cassette]` — present ⇒ the feature is wired; absent (or
    /// `enabled = false`) ⇒ it is **omitted from the chain**, not added as a
    /// no-op (zero overhead when disabled).
    ///
    /// `[auth]` wires the static auth strategy. OAuth token-refresh auth is a
    /// runtime `token_source` (a trait object), not a config section, so it is
    /// not activated here — supply it through the explicit `http_egress` /
    /// `http_egress_oauth` factories.
    ///
    /// ```toml
    /// # enabling TLS is all the consumer writes:
    /// [tls]
    /// kind = "pem"
    /// path = "/etc/certs/client.pem"
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`HttpEgressBuildError::Config`] if a section fails to load or
    /// validate, or the relevant middleware build error if assembly fails.
    pub fn http_egress_from_config(
        loader: &swe_edge_configbuilder::SectionLoaderImpl,
    ) -> Result<Box<dyn HttpEgress>, HttpEgressBuildError> {
        use swe_edge_configbuilder::{FeatureState, OptionalSection as _};

        let http_cfg = HttpConfig::default();
        let mut cb = reqwest::Client::builder();

        // [tls] present ⇒ build and apply the TLS layer; absent ⇒ no TLS layer is
        // constructed or applied (the disabled feature adds nothing).
        if let FeatureState::Enabled(tls_cfg) =
            swe_edge_egress_tls::TlsConfig::load_optional(loader)?
        {
            let tls = swe_edge_egress_tls::HttpTlsSvc::build_tls_layer(tls_cfg)?;
            cb = tls.apply_to(cb)?;
        }

        cb = Self::configure_http_builder(cb, &http_cfg);
        let reqwest_client = cb.build()?;

        // Each enabled [section] adds exactly one middleware via `.with(..)`.
        // A section that is absent (or `enabled = false`) adds nothing — no
        // middleware in the chain, not a no-op layer.
        let mut builder = ClientBuilder::new(reqwest_client);

        // [auth] present ⇒ add the static auth layer. OAuth token-refresh is a
        // runtime token_source (a trait object), not a config section, so it is
        // wired programmatically rather than via [auth].
        if let FeatureState::Enabled(auth_cfg) =
            swe_edge_egress_auth::AuthConfig::load_optional(loader)?
        {
            let auth = swe_edge_egress_auth::AuthSvc::build_auth_middleware(auth_cfg)?;
            builder = builder.with(auth);
        }

        // [retry] present ⇒ add the retry policy layer.
        if let FeatureState::Enabled(retry_cfg) =
            swe_edge_egress_retry::RetryConfig::load_optional(loader)?
        {
            let retry = swe_edge_egress_retry::HttpRetrySvc::build_retry_layer(retry_cfg)?;
            builder = builder.with(retry);
        }

        // [rate] present ⇒ add the rate-limiting layer.
        if let FeatureState::Enabled(rate_cfg) =
            swe_edge_egress_rate::RateConfig::load_optional(loader)?
        {
            let rate = swe_edge_egress_rate::HttpRateSvc::build_rate_layer(rate_cfg)?;
            builder = builder.with(rate);
        }

        // [breaker] present ⇒ add the circuit-breaker layer.
        if let FeatureState::Enabled(breaker_cfg) =
            swe_edge_egress_breaker::BreakerConfig::load_optional(loader)?
        {
            let breaker =
                swe_edge_egress_breaker::HttpBreakerSvc::build_breaker_layer(breaker_cfg)?;
            builder = builder.with(breaker);
        }

        // [cache] present ⇒ add the response-cache layer.
        if let FeatureState::Enabled(cache_cfg) =
            swe_edge_egress_cache::CacheConfig::load_optional(loader)?
        {
            let cache = swe_edge_egress_cache::HttpCacheSvc::build_cache_layer(cache_cfg)?;
            builder = builder.with(cache);
        }

        // [cassette] present ⇒ add the record/replay layer (default fixture name).
        if let FeatureState::Enabled(cassette_cfg) =
            swe_edge_egress_cassette::CassetteConfig::load_optional(loader)?
        {
            let cassette = swe_edge_egress_cassette::HttpCassetteSvc::build_cassette_layer(
                cassette_cfg,
                "default",
            )?;
            builder = builder.with(cassette);
        }

        let client = builder.build();

        Ok(Box::new(DefaultHttpEgress::new(
            client,
            http_cfg.base_url,
            http_cfg.max_response_bytes,
        )))
    }

    /// Dry-run the config-driven egress: load every optional `[section]` into a
    /// [`FeatureRegistry`] and return a [`FeatureSummary`] of what would
    /// activate — without building any middleware. Log this at startup so
    /// operators see exactly which features are on (and why); it is the
    /// visibility guardrail against silent config-driven activation.
    ///
    /// Mirrors the section set of [`http_egress_from_config`]: `[auth]`,
    /// `[tls]`, `[retry]`, `[rate]`, `[breaker]`, `[cache]`, `[cassette]`.
    ///
    /// # Errors
    ///
    /// Returns [`HttpEgressBuildError::Config`] if a present section fails to
    /// parse or validate.
    ///
    /// [`FeatureRegistry`]: swe_edge_configbuilder::FeatureRegistry
    /// [`FeatureSummary`]: swe_edge_configbuilder::FeatureSummary
    pub fn preflight(
        loader: &swe_edge_configbuilder::SectionLoaderImpl,
    ) -> Result<swe_edge_configbuilder::FeatureSummary, HttpEgressBuildError> {
        let mut registry = swe_edge_configbuilder::FeatureRegistry::new();
        registry.load::<swe_edge_egress_auth::AuthConfig>(loader)?;
        registry.load::<swe_edge_egress_tls::TlsConfig>(loader)?;
        registry.load::<swe_edge_egress_retry::RetryConfig>(loader)?;
        registry.load::<swe_edge_egress_rate::RateConfig>(loader)?;
        registry.load::<swe_edge_egress_breaker::BreakerConfig>(loader)?;
        registry.load::<swe_edge_egress_cache::CacheConfig>(loader)?;
        registry.load::<swe_edge_egress_cassette::CassetteConfig>(loader)?;
        Ok(registry.summary())
    }

    /// Build an [`HttpEgress`] with OAuth token-refresh auth and SWE defaults
    /// for every other middleware layer.
    ///
    /// Shorthand for `http_egress` when the caller supplies an
    /// [`OAuthTokenSource`] and accepts the SWE defaults for retry, rate,
    /// breaker, cache, cassette, and TLS.
    pub fn http_egress_oauth(
        http: HttpConfig,
        source: Arc<dyn swe_edge_egress_oauth::OAuthTokenSource>,
    ) -> Result<Box<dyn HttpEgress>, HttpEgressBuildError> {
        let oauth = swe_edge_egress_oauth::OAuthSvc::builder()
            .with_token_source(source)
            .build()?;
        Ok(Box::new(Self::assemble(
            http,
            oauth,
            swe_edge_egress_retry::HttpRetrySvc::build_retry_layer(Default::default())?,
            swe_edge_egress_rate::HttpRateSvc::build_rate_layer(Default::default())?,
            swe_edge_egress_breaker::HttpBreakerSvc::build_breaker_layer(Default::default())?,
            swe_edge_egress_cache::HttpCacheSvc::build_cache_layer(Default::default())?,
            // Cassette is disabled in production convenience functions — it is
            // test infrastructure and must not intercept real outbound calls.
            swe_edge_egress_cassette::HttpCassetteSvc::build_cassette_layer(
                swe_edge_egress_cassette::CassetteConfig::disabled(),
                "unused",
            )?,
            swe_edge_egress_tls::HttpTlsSvc::build_tls_layer(Default::default())?,
        )?))
    }

    /// Build an [`HttpEgress`] with a static [`AuthConfig`] and SWE defaults
    /// for every other middleware layer.
    ///
    /// Shorthand for `http_egress` when the caller uses an env-var-backed
    /// credential (Bearer, Header, Basic, etc.) and accepts the SWE defaults.
    pub fn http_egress_with_auth(
        http: HttpConfig,
        auth: swe_edge_egress_auth::AuthConfig,
    ) -> Result<Box<dyn HttpEgress>, HttpEgressBuildError> {
        let auth_mw = swe_edge_egress_auth::AuthSvc::build_auth_middleware(auth)?;
        let retry = swe_edge_egress_retry::HttpRetrySvc::build_retry_layer(Default::default())?;
        let rate = swe_edge_egress_rate::HttpRateSvc::build_rate_layer(Default::default())?;
        let breaker =
            swe_edge_egress_breaker::HttpBreakerSvc::build_breaker_layer(Default::default())?;
        let cache = swe_edge_egress_cache::HttpCacheSvc::build_cache_layer(Default::default())?;
        let cassette = swe_edge_egress_cassette::HttpCassetteSvc::build_cassette_layer(
            swe_edge_egress_cassette::CassetteConfig::disabled(),
            "unused",
        )?;
        let tls = swe_edge_egress_tls::HttpTlsSvc::build_tls_layer(Default::default())?;
        Ok(Box::new(Self::assemble(
            http, auth_mw, retry, rate, breaker, cache, cassette, tls,
        )?))
    }

    /// Build an [`HttpEgress`] using the SWE-shipped defaults for every
    /// middleware layer (pass-through auth, no TLS, sensible retry/rate/breaker
    /// policies from each crate's `config/application.toml`).
    pub fn default_http_egress() -> Result<Box<dyn HttpEgress>, HttpEgressBuildError> {
        let auth = swe_edge_egress_auth::AuthSvc::build_auth_middleware(Default::default())?;
        let retry = swe_edge_egress_retry::HttpRetrySvc::build_retry_layer(Default::default())?;
        let rate = swe_edge_egress_rate::HttpRateSvc::build_rate_layer(Default::default())?;
        let breaker =
            swe_edge_egress_breaker::HttpBreakerSvc::build_breaker_layer(Default::default())?;
        let cache = swe_edge_egress_cache::HttpCacheSvc::build_cache_layer(Default::default())?;
        let cassette = swe_edge_egress_cassette::HttpCassetteSvc::build_cassette_layer(
            Default::default(),
            "default",
        )?;
        let tls = swe_edge_egress_tls::HttpTlsSvc::build_tls_layer(Default::default())?;
        Ok(Box::new(Self::assemble(
            HttpConfig::default(),
            auth,
            retry,
            rate,
            breaker,
            cache,
            cassette,
            tls,
        )?))
    }

    /// Build a fully-assembled [`HttpEgress`] using the provided [`HttpConfig`]
    /// and SWE defaults for every middleware layer.
    ///
    /// Same middleware stack as [`default_http_egress`] but with caller-supplied
    /// transport settings (base URL, timeouts, headers, etc.).
    pub fn default_http_egress_with_config(
        http: HttpConfig,
    ) -> Result<Box<dyn HttpEgress>, HttpEgressBuildError> {
        let auth = swe_edge_egress_auth::AuthSvc::build_auth_middleware(Default::default())?;
        let retry = swe_edge_egress_retry::HttpRetrySvc::build_retry_layer(Default::default())?;
        let rate = swe_edge_egress_rate::HttpRateSvc::build_rate_layer(Default::default())?;
        let breaker =
            swe_edge_egress_breaker::HttpBreakerSvc::build_breaker_layer(Default::default())?;
        let cache = swe_edge_egress_cache::HttpCacheSvc::build_cache_layer(Default::default())?;
        let cassette = swe_edge_egress_cassette::HttpCassetteSvc::build_cassette_layer(
            swe_edge_egress_cassette::CassetteConfig::disabled(),
            "unused",
        )?;
        let tls = swe_edge_egress_tls::HttpTlsSvc::build_tls_layer(Default::default())?;
        Ok(Box::new(Self::assemble(
            http, auth, retry, rate, breaker, cache, cassette, tls,
        )?))
    }

    /// Wrap any [`HttpEgress`] with per-call metrics observation.
    ///
    /// Consumers call this after any of the factory functions to add observability
    /// without changing how the outbound is configured:
    ///
    /// ```rust,ignore
    /// let outbound = HttpTransportSvc::observe_http_egress(default_http_egress()?, metrics_provider);
    /// ```
    /// Wrap any [`HttpEgress`] with per-call metrics observation.
    ///
    /// Consumers call this after any of the factory functions to add observability
    /// without changing how the outbound is configured:
    ///
    /// ```rust,ignore
    /// let outbound = HttpTransportSvc::observe_http_egress(default_http_egress()?, metrics_provider);
    /// ```
    pub fn observe_http_egress(
        inner: Box<dyn HttpEgress>,
        provider: Arc<dyn MetricsProvider>,
    ) -> Box<dyn HttpEgress> {
        let arc_inner: Arc<dyn HttpEgress> = Arc::from(inner);
        Box::new(MetricsHttpEgress::new(arc_inner, provider))
    }

    /// Build a fully-assembled [`HttpStream`] using the SWE defaults.
    ///
    /// Returns the same default middleware stack as [`default_http_egress`]
    /// but typed as [`HttpStream`], so callers can use SSE and WebSocket
    /// features without importing or naming the concrete type.
    pub fn default_http_stream_outbound() -> Result<Box<dyn HttpStream>, HttpEgressBuildError> {
        let auth = swe_edge_egress_auth::AuthSvc::build_auth_middleware(Default::default())?;
        let retry = swe_edge_egress_retry::HttpRetrySvc::build_retry_layer(Default::default())?;
        let rate = swe_edge_egress_rate::HttpRateSvc::build_rate_layer(Default::default())?;
        let breaker =
            swe_edge_egress_breaker::HttpBreakerSvc::build_breaker_layer(Default::default())?;
        let cache = swe_edge_egress_cache::HttpCacheSvc::build_cache_layer(Default::default())?;
        let cassette = swe_edge_egress_cassette::HttpCassetteSvc::build_cassette_layer(
            Default::default(),
            "default",
        )?;
        let tls = swe_edge_egress_tls::HttpTlsSvc::build_tls_layer(Default::default())?;
        Ok(Box::new(Self::assemble(
            HttpConfig::default(),
            auth,
            retry,
            rate,
            breaker,
            cache,
            cassette,
            tls,
        )?))
    }

    /// Build a minimal [`HttpEgress`] from just an [`HttpConfig`] — no middleware layers.
    ///
    /// All `HttpConfig` fields are honoured: `timeout_secs`, `connect_timeout_secs`,
    /// `user_agent`, `follow_redirects`, `max_redirects`, `default_headers`, and
    /// `max_response_bytes`.  Useful for integration tests and simple deployments
    /// that do not need the full auth/retry/rate/breaker/cache/cassette stack.
    pub fn plain_http_egress(
        config: HttpConfig,
    ) -> Result<Box<dyn HttpEgress>, HttpEgressBuildError> {
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
        Ok(Box::new(DefaultHttpEgress::new(
            client,
            config.base_url,
            config.max_response_bytes,
        )))
    }

    /// Validate that an [`HttpConfig`] value is well-formed.
    ///
    /// Returns `Ok(())` when the config is valid, or a human-readable error
    /// message explaining what field is invalid and what the expected constraint is.
    pub fn validate_http_config(config: &HttpConfig) -> Result<(), String> {
        use crate::core::validator::{DefaultValidator, HttpConfigValidator};
        DefaultValidator::new(HttpConfigValidator::new(config)).validate()
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
    ) -> Result<DefaultHttpEgress, HttpEgressBuildError> {
        let mut cb = reqwest::Client::builder();
        cb = tls.apply_to(cb)?;
        cb = Self::configure_http_builder(cb, &http_cfg);
        let reqwest_client = cb.build()?;

        let client = ClientBuilder::new(reqwest_client)
            .with(auth)
            .with(retry)
            .with(rate)
            .with(breaker)
            .with(cache)
            .with(cassette)
            .build();

        Ok(DefaultHttpEgress::new(
            client,
            http_cfg.base_url,
            http_cfg.max_response_bytes,
        ))
    }

    /// Apply [`HttpConfig`] transport settings — timeouts, user-agent, redirect
    /// policy, and default headers — to a reqwest client builder. Shared by the
    /// explicit `assemble` path and the config-driven `http_egress_from_config`.
    fn configure_http_builder(
        mut cb: reqwest::ClientBuilder,
        http_cfg: &HttpConfig,
    ) -> reqwest::ClientBuilder {
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
        cb
    }
}
