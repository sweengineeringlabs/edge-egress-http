//! SAF layer — public facade.
//!
//! `builder()` is the sole public entry point. Callers supply their
//! own `OAuthTokenSource` implementation; this crate only provides the
//! middleware shell that wraps it.

use std::sync::Arc;

use crate::core::OAuthRefreshStrategy;

pub use crate::api::{OAuthBuilderOps, OAuthError, OAuthTokenSource, Result};

// ── OAuthMiddleware ───────────────────────────────────────────────────────────

/// reqwest-middleware layer that injects a proactively-refreshed OAuth bearer
/// token (`Authorization: Bearer <token>`) on every outbound request.
///
/// Construct via [`builder()`] → [`Builder::with_token_source`] → [`Builder::build`].
///
/// ```ignore
/// let mw = swe_edge_egress_oauth::builder()
///     .with_token_source(my_token_source)
///     .build()?;
///
/// let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
///     .with(mw)
///     .build();
/// ```
pub struct OAuthMiddleware {
    pub(crate) inner: Arc<OAuthRefreshStrategy>,
}

impl std::fmt::Debug for OAuthMiddleware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OAuthMiddleware").finish_non_exhaustive()
    }
}

#[async_trait::async_trait]
impl reqwest_middleware::Middleware for OAuthMiddleware {
    async fn handle(
        &self,
        req: reqwest::Request,
        ext: &mut http::Extensions,
        next: reqwest_middleware::Next<'_>,
    ) -> reqwest_middleware::Result<reqwest::Response> {
        self.inner.handle(req, ext, next).await
    }
}

// ── Builder ───────────────────────────────────────────────────────────────────

/// Fluent builder for [`OAuthMiddleware`].
#[derive(Default)]
pub struct Builder {
    source: Option<Arc<dyn OAuthTokenSource>>,
}

impl OAuthBuilderOps for Builder {
    type Middleware = OAuthMiddleware;

    fn with_token_source(mut self, source: Arc<dyn OAuthTokenSource>) -> Self {
        self.source = Some(source);
        self
    }

    fn build(self) -> std::result::Result<OAuthMiddleware, OAuthError> {
        let source = self.source.ok_or_else(|| {
            OAuthError::Configuration(
                "no OAuthTokenSource provided — call with_token_source first".into(),
            )
        })?;
        Ok(OAuthMiddleware {
            inner: Arc::new(OAuthRefreshStrategy::new(source)),
        })
    }
}

/// Create a builder for [`OAuthMiddleware`].
pub fn builder() -> impl OAuthBuilderOps<Middleware = OAuthMiddleware> {
    Builder::default()
}

// ── re-exports ────────────────────────────────────────────────────────────────

pub use crate::api::{OAuthConfig, OAuthCredentials, OAuthProvider};
