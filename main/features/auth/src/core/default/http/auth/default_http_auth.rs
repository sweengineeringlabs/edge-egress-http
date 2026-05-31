//! Default impl of [`HttpAuth`](crate::api::traits::http_auth::HttpAuth).
//!
//! Holds a pre-resolved [`AuthStrategy`] (constructed once at
//! `build()` time from the config + resolver) and delegates
//! `process()` to it on every request.

use futures::future::BoxFuture;

use crate::api::auth::auth_config::AuthConfig;
use crate::api::auth::auth_strategy::AuthStrategy;
use crate::api::error::AuthError;
use crate::api::traits::credential_resolver::CredentialResolver;
use crate::api::traits::http_auth::HttpAuth;
use crate::api::traits::{Processor, Validator};
use crate::core::strategy::strategy_factory::StrategyFactory;

/// Default HTTP auth processor. Holds the resolved strategy;
/// per-request work is just `strategy.authorize(req)`.
pub(crate) struct DefaultHttpAuth {
    /// Kept for `describe()` / diagnostics — the config kind as
    /// declared, before resolution.
    config_kind: &'static str,
    /// Pre-resolved strategy realizing the config.
    strategy: Box<dyn AuthStrategy>,
}

impl std::fmt::Debug for DefaultHttpAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefaultHttpAuth")
            .field("config_kind", &self.config_kind)
            .field("strategy", &self.strategy)
            .finish()
    }
}

impl DefaultHttpAuth {
    /// Build from a config + credential resolver. Resolves all
    /// env-var references NOW — startup fails with
    /// [`AuthError::MissingEnvVar`] if any referenced variable is
    /// unset.
    pub(crate) fn new(
        config: AuthConfig,
        resolver: &dyn CredentialResolver,
    ) -> Result<Self, AuthError> {
        let config_kind = match &config {
            AuthConfig::None => "none",
            AuthConfig::Bearer { .. } => "bearer",
            AuthConfig::Basic { .. } => "basic",
            AuthConfig::Header { .. } => "header",
            AuthConfig::AwsSigV4 { .. } => "aws_sigv4",
        };
        let strategy = StrategyFactory::build_strategy(&config, resolver)?;
        Ok(Self {
            config_kind,
            strategy,
        })
    }
}

impl HttpAuth for DefaultHttpAuth {
    fn describe(&self) -> &'static str {
        "swe_edge_egress_auth"
    }

    fn process<'a>(
        &'a self,
        req: &'a mut reqwest::Request,
    ) -> BoxFuture<'a, Result<(), AuthError>> {
        Box::pin(async move {
            // Two-phase: first, any strategy-specific setup (Digest
            // fetches nonce here), then attach header.
            let host = req.url().host_str();
            self.strategy.prepare(host).await?;
            self.strategy.authorize(req)
        })
    }
}

impl Processor for DefaultHttpAuth {
    fn describe(&self) -> &'static str {
        // Delegates to HttpAuth::describe for consistency.
        HttpAuth::describe(self)
    }

    fn process<'a>(
        &'a self,
        req: &'a mut reqwest::Request,
    ) -> BoxFuture<'a, Result<(), AuthError>> {
        HttpAuth::process(self, req)
    }
}

impl Validator for DefaultHttpAuth {
    fn validate(&self) -> Result<(), AuthError> {
        // All validation happens at construction time in `new()`.
        // A constructed `DefaultHttpAuth` is always valid.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::types::credential_source::CredentialSource;
    use secrecy::SecretString;

    /// Test resolver that returns a fixed credential for all sources.
    struct DefaultHttpAuthStubResolver(&'static str);
    impl CredentialResolver for DefaultHttpAuthStubResolver {
        fn resolve(&self, _source: &CredentialSource) -> Result<SecretString, AuthError> {
            Ok(SecretString::from(self.0.to_string()))
        }
    }

    fn stub_request() -> reqwest::Request {
        reqwest::Request::new(
            reqwest::Method::GET,
            reqwest::Url::parse("http://example.test/").unwrap(),
        )
    }

    /// @covers: describe
    #[test]
    fn test_describe_returns_crate_name() {
        let cfg = AuthConfig::None;
        let d = DefaultHttpAuth::new(cfg, &DefaultHttpAuthStubResolver("x")).expect("build ok");
        assert_eq!(HttpAuth::describe(&d), "swe_edge_egress_auth");
    }

    /// @covers: process
    #[tokio::test]
    async fn test_process_with_none_config_adds_no_header() {
        let d = DefaultHttpAuth::new(AuthConfig::None, &DefaultHttpAuthStubResolver("x")).unwrap();
        let mut req = stub_request();
        HttpAuth::process(&d, &mut req).await.unwrap();
        assert!(req.headers().get("authorization").is_none());
    }

    /// @covers: process
    #[tokio::test]
    async fn test_process_with_bearer_config_attaches_authorization() {
        let cfg = AuthConfig::Bearer {
            token_env: "X".into(),
        };
        let d = DefaultHttpAuth::new(cfg, &DefaultHttpAuthStubResolver("tok-7")).unwrap();
        let mut req = stub_request();
        HttpAuth::process(&d, &mut req).await.unwrap();
        assert_eq!(
            req.headers()
                .get("authorization")
                .unwrap()
                .to_str()
                .unwrap(),
            "Bearer tok-7"
        );
    }

    /// @covers: fmt
    #[test]
    fn test_fmt_debug_contains_struct_name_and_not_credentials() {
        let cfg = AuthConfig::Bearer {
            token_env: "X".into(),
        };
        let d = DefaultHttpAuth::new(cfg, &DefaultHttpAuthStubResolver("secret-tok")).unwrap();
        let s = format!("{d:?}");
        assert!(s.contains("DefaultHttpAuth"), "debug output: {s}");
        // The resolved token must not leak into the debug output.
        assert!(!s.contains("secret-tok"), "token leaked into debug: {s}");
    }

    /// @covers: new
    #[test]
    fn test_new_constructs_successfully_with_none_config() {
        let d = DefaultHttpAuth::new(AuthConfig::None, &DefaultHttpAuthStubResolver("x"));
        assert!(d.is_ok());
    }

    /// @covers: new
    #[test]
    fn test_build_fails_fast_on_missing_env_var() {
        struct DefaultHttpAuthMissingResolver;
        impl CredentialResolver for DefaultHttpAuthMissingResolver {
            fn resolve(&self, source: &CredentialSource) -> Result<SecretString, AuthError> {
                match source {
                    CredentialSource::EnvVar(n) => {
                        Err(AuthError::MissingEnvVar { name: n.clone() })
                    }
                }
            }
        }
        let cfg = AuthConfig::Bearer {
            token_env: "NOT_SET".into(),
        };
        let err = DefaultHttpAuth::new(cfg, &DefaultHttpAuthMissingResolver).unwrap_err();
        match err {
            AuthError::MissingEnvVar { name } => assert_eq!(name, "NOT_SET"),
            other => panic!("expected MissingEnvVar, got {other:?}"),
        }
    }
}
