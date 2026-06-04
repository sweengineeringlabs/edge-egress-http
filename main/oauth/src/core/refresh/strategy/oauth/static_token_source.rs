//! `StaticTokenSource` — test double that returns a fixed token.

use futures::future::BoxFuture;

use crate::api::oauth::o_auth_token_source::OAuthTokenSource;

/// A token source that always returns the same static token. Used in tests.
pub(crate) struct StaticTokenSource(pub(crate) String);

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
