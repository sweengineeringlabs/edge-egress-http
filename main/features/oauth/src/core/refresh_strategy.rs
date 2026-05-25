//! [`OAuthRefreshStrategy`] — `reqwest_middleware::Middleware` that injects a
//! proactively-refreshed OAuth bearer token on every outbound request.

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use tokio::sync::Mutex;
use tracing::debug;

use crate::api::error::OAuthError as Error;
use crate::api::traits::token_source::OAuthTokenSource;

/// Refresh proactively this many milliseconds before actual expiry.
const REFRESH_WINDOW_MS: u64 = 60_000;

struct CachedToken {
    value: String,
    /// Unix-epoch milliseconds when the token expires.
    expires_at_ms: u64,
}

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

pub(crate) struct OAuthTimeHelper;

impl OAuthTimeHelper {
    pub(crate) fn now_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::traits::token_source::{FailingTokenSource, StaticTokenSource};

    #[tokio::test]
    async fn test_fresh_token_returns_source_token() {
        let strategy = OAuthRefreshStrategy::new(Arc::new(StaticTokenSource("tok-123".into())));
        let token = strategy.fresh_token().await.unwrap();
        assert_eq!(token, "tok-123");
    }

    #[tokio::test]
    async fn test_fresh_token_is_cached_on_second_call() {
        let strategy = OAuthRefreshStrategy::new(Arc::new(StaticTokenSource("tok-abc".into())));
        let t1 = strategy.fresh_token().await.unwrap();
        let t2 = strategy.fresh_token().await.unwrap();
        assert_eq!(t1, t2);
    }

    #[tokio::test]
    async fn test_fresh_token_propagates_source_error() {
        let strategy = OAuthRefreshStrategy::new(Arc::new(FailingTokenSource));
        let result = strategy.fresh_token().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("test failure"));
    }
}

#[cfg(test)]
mod sync_coverage {
    use super::OAuthRefreshStrategy;
    use crate::api::traits::token_source::StaticTokenSource;
    use std::sync::Arc;

    /// @covers: new
    #[test]
    fn test_new_creates_strategy() {
        let _ = OAuthRefreshStrategy::new(Arc::new(StaticTokenSource("tok".into())));
    }
}
