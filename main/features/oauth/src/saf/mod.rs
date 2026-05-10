//! SAF layer — public facade.
//!
//! `builder()` is the sole public entry point. Callers supply their
//! own `OAuthTokenSource` implementation; this crate only provides the
//! middleware shell that wraps it.

use std::sync::Arc;

use crate::core::OAuthRefreshStrategy;

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

impl Builder {
    /// Set the token source implementation.
    pub fn with_token_source(mut self, source: Arc<dyn OAuthTokenSource>) -> Self {
        self.source = Some(source);
        self
    }

    /// Build the middleware.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Configuration`] if no token source was provided.
    pub fn build(self) -> std::result::Result<OAuthMiddleware, crate::api::Error> {
        let source = self.source.ok_or_else(|| {
            crate::api::Error::Configuration(
                "no OAuthTokenSource provided — call with_token_source first".into(),
            )
        })?;
        Ok(OAuthMiddleware {
            inner: Arc::new(OAuthRefreshStrategy::new(source)),
        })
    }
}

/// Create a [`Builder`] with no defaults.
pub fn builder() -> Builder {
    Builder::default()
}

// ── re-exports ────────────────────────────────────────────────────────────────

pub use crate::api::{Error, OAuthTokenSource, Result};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::token_source::StaticTokenSource;

    #[test]
    fn test_builder_without_source_returns_error() {
        let result = builder().build();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("no OAuthTokenSource"));
    }

    #[test]
    fn test_builder_with_source_builds_middleware() {
        let src = Arc::new(StaticTokenSource("tok".into()));
        let result = builder().with_token_source(src).build();
        assert!(result.is_ok());
    }
}
