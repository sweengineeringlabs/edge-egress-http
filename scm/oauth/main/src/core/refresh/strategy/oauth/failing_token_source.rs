//! `FailingTokenSource` — test double that always returns an error.

use futures::future::BoxFuture;

use crate::api::traits::OAuthTokenSource;

/// A token source that always fails. Used in tests to verify error propagation.
pub(crate) struct FailingTokenSource;

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
