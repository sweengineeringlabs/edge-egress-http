//! Builder trait for the OAuth middleware.

use std::sync::Arc;

use crate::api::error::OAuthError;
use crate::api::traits::token_source::OAuthTokenSource;

/// Fluent builder contract for the OAuth middleware.
///
/// `Output` is the concrete middleware type produced by `build()`;
/// the associated type keeps this trait free of circular dependencies
/// with the saf/ layer.
pub trait OAuthBuilderOps {
    /// The middleware type produced by this builder.
    type Middleware;

    /// Set the token source implementation.
    fn with_token_source(self, source: Arc<dyn OAuthTokenSource>) -> Self;

    /// Build the middleware.
    ///
    /// # Errors
    /// Returns [`OAuthError::Configuration`] if no token source was provided.
    fn build(self) -> std::result::Result<Self::Middleware, OAuthError>;
}
