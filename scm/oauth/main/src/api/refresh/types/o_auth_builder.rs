//! `OAuthBuilder` — fluent builder for [`OAuthMiddleware`].

use std::sync::Arc;

use crate::api::refresh::errors::OAuthError;
use crate::api::refresh::traits::OAuthBuilderOps;
use crate::api::refresh::traits::OAuthTokenSource;
use crate::api::refresh::types::OAuthMiddleware;

/// Fluent builder for [`OAuthMiddleware`].
#[derive(Default)]
pub struct OAuthBuilder {
    pub(crate) source: Option<Arc<dyn OAuthTokenSource>>,
}

impl OAuthBuilderOps for OAuthBuilder {
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
        let strategy = crate::core::refresh::strategy::OAuthRefreshStrategy::new(source);
        Ok(OAuthMiddleware {
            inner: Arc::new(strategy),
        })
    }
}

impl OAuthBuilder {
    /// Create an empty builder with no token source configured.
    pub fn new() -> Self {
        Self { source: None }
    }
}
