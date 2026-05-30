//! `OAuthMiddleware` — reqwest-middleware layer that injects a bearer token.

use std::sync::Arc;

/// reqwest-middleware layer that injects a proactively-refreshed OAuth bearer
/// token (`Authorization: Bearer <token>`) on every outbound request.
///
/// Construct via [`crate::saf::OAuthSvc::builder`].
pub struct OAuthMiddleware {
    pub(crate) inner: Arc<dyn reqwest_middleware::Middleware>,
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
