//! [`OAuthRefreshStrategy`] — `reqwest_middleware::Middleware` that injects a
//! proactively-refreshed OAuth bearer token on every outbound request.
use super::cached_token::{CachedToken, REFRESH_WINDOW_MS};

use std::sync::Arc;

use async_trait::async_trait;
use futures::future::BoxFuture;
use tokio::sync::Mutex;
use tracing::debug;

use super::super::OAuthTimeHelper;
use crate::api::error::OAuthError as Error;
use crate::api::oauth_credentials::OAuthCredentials;
use crate::api::oauth_token_source::OAuthTokenSource;
use crate::api::traits::{Processor, Validator};

/// Refresh proactively this many milliseconds before actual expiry.

/// reqwest-middleware layer that injects `Authorization: Bearer <token>` on
/// every outbound request, refreshing the token proactively before it expires.
///
/// Concurrent callers serialize on the mutex rather than racing to refresh.
pub(crate) struct OAuthRefreshStrategy {
    source: Arc<dyn OAuthTokenSource>,
    cached: Mutex<Option<CachedToken>>,
}

impl OAuthRefreshStrategy {
    pub(crate) fn new(source: Arc<dyn OAuthTokenSource>) -> Self {
        Self {
            source,
            cached: Mutex::new(None),
        }
    }

    async fn fresh_token(&self) -> Result<String, Error> {
        let now = OAuthTimeHelper::now_ms();
        let mut guard = self.cached.lock().await;

        let needs_refresh = match guard.as_ref() {
            None => true,
            Some(c) => c.expires_at_ms.saturating_sub(now) < REFRESH_WINDOW_MS,
        };

        if needs_refresh {
            debug!("OAuth token expired or missing — refreshing");
            // token_source is responsible for hitting the token endpoint and
            // returning a fresh token; we cache it with a 1-hour TTL as a
            // fallback if the source doesn't know the expiry.
            let token = self.source.get_access_token().await?;
            *guard = Some(CachedToken {
                value: token.clone(),
                expires_at_ms: now + 3_600_000, // 1-hour fallback
            });
            Ok(token)
        } else if let Some(cached) = guard.as_ref() {
            Ok(cached.value.clone())
        } else {
            Err(Error::RefreshFailed(
                "cached token missing despite fresh check".into(),
            ))
        }
    }
}

impl std::fmt::Debug for OAuthRefreshStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OAuthRefreshStrategy")
            .finish_non_exhaustive()
    }
}

impl Processor for OAuthRefreshStrategy {
    fn process(&self) -> BoxFuture<'_, crate::api::error::Result<String>> {
        Box::pin(self.fresh_token())
    }
}

impl Validator for OAuthRefreshStrategy {
    fn validate(credentials: &OAuthCredentials) -> crate::api::error::Result<()> {
        if credentials.access_token.is_empty() {
            return Err(Error::Configuration(
                "access_token must not be empty".into(),
            ));
        }
        if credentials.refresh_token.is_empty() {
            return Err(Error::Configuration(
                "refresh_token must not be empty".into(),
            ));
        }
        Ok(())
    }
}

#[async_trait]
impl reqwest_middleware::Middleware for OAuthRefreshStrategy {
    async fn handle(
        &self,
        mut req: reqwest::Request,
        ext: &mut http::Extensions,
        next: reqwest_middleware::Next<'_>,
    ) -> reqwest_middleware::Result<reqwest::Response> {
        let token = self
            .fresh_token()
            .await
            .map_err(|e| reqwest_middleware::Error::Middleware(anyhow::anyhow!("{}", e)))?;

        let auth_value = reqwest::header::HeaderValue::from_str(&format!("Bearer {token}"))
            .map_err(|e| {
                reqwest_middleware::Error::Middleware(anyhow::anyhow!("invalid bearer token: {e}"))
            })?;
        req.headers_mut()
            .insert(reqwest::header::AUTHORIZATION, auth_value);

        next.run(req, ext).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct StaticTokenSource(String);

    impl OAuthTokenSource for StaticTokenSource {
        fn get_access_token(&self) -> BoxFuture<'_, crate::api::error::Result<String>> {
            let v = self.0.clone();
            Box::pin(async move { Ok(v) })
        }
    }

    impl std::fmt::Debug for StaticTokenSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("StaticTokenSource").finish()
        }
    }

    struct FailingTokenSource;

    impl OAuthTokenSource for FailingTokenSource {
        fn get_access_token(&self) -> BoxFuture<'_, crate::api::error::Result<String>> {
            Box::pin(async {
                Err(crate::api::error::OAuthError::RefreshFailed(
                    "test failure".into(),
                ))
            })
        }
    }

    impl std::fmt::Debug for FailingTokenSource {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("FailingTokenSource").finish()
        }
    }

    /// @covers: new
    #[test]
    fn test_new_creates_strategy() {
        let _ = OAuthRefreshStrategy::new(Arc::new(StaticTokenSource("tok".into())));
    }

    /// @covers: fresh_token
    #[tokio::test]
    async fn test_fresh_token_returns_source_token() {
        let strategy = OAuthRefreshStrategy::new(Arc::new(StaticTokenSource("tok-123".into())));
        let token = strategy.fresh_token().await.unwrap();
        assert_eq!(token, "tok-123");
    }

    /// @covers: fresh_token
    #[tokio::test]
    async fn test_fresh_token_is_cached_on_second_call() {
        let strategy = OAuthRefreshStrategy::new(Arc::new(StaticTokenSource("tok-abc".into())));
        let t1 = strategy.fresh_token().await.unwrap();
        let t2 = strategy.fresh_token().await.unwrap();
        assert_eq!(t1, t2);
    }

    /// @covers: fresh_token
    #[tokio::test]
    async fn test_fresh_token_propagates_source_error() {
        let strategy = OAuthRefreshStrategy::new(Arc::new(FailingTokenSource));
        let result = strategy.fresh_token().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("test failure"));
    }

    /// @covers: validate
    #[test]
    fn test_validate_rejects_empty_access_token() {
        let creds = OAuthCredentials {
            access_token: String::new(),
            refresh_token: "rt".into(),
            expires_at_ms: 0,
            scopes: vec![],
        };
        assert!(OAuthRefreshStrategy::validate(&creds).is_err());
    }

    /// @covers: validate
    #[test]
    fn test_validate_rejects_empty_refresh_token() {
        let creds = OAuthCredentials {
            access_token: "at".into(),
            refresh_token: String::new(),
            expires_at_ms: 0,
            scopes: vec![],
        };
        assert!(OAuthRefreshStrategy::validate(&creds).is_err());
    }

    /// @covers: validate
    #[test]
    fn test_validate_accepts_well_formed_credentials() {
        let creds = OAuthCredentials {
            access_token: "at".into(),
            refresh_token: "rt".into(),
            expires_at_ms: 1_000_000,
            scopes: vec!["read".into()],
        };
        assert!(OAuthRefreshStrategy::validate(&creds).is_ok());
    }
}
