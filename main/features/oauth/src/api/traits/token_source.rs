//! `OAuthTokenSource` — public extension point for OAuth token providers.
//!
//! Implement this trait to plug your own credential store into the edge
//! OAuth middleware. The middleware calls `get_access_token()` on every
//! outbound request; your implementation is responsible for proactive
//! refresh and any credential file I/O.

use futures::future::BoxFuture;

use crate::api::error::Result;

/// Provides a valid OAuth2 access token on demand.
///
/// Implementations are responsible for:
/// - Loading credentials from their backing store (file, env, secret manager …)
/// - Proactively refreshing the token before it expires
/// - Persisting refreshed credentials back to the store
///
/// The middleware calls this once per request; implementations should cache
/// the current token in memory and only hit the token endpoint when necessary.
pub trait OAuthTokenSource: Send + Sync + std::fmt::Debug + 'static {
    /// Return a valid access token. Must not return an expired token.
    fn get_access_token(&self) -> BoxFuture<'_, Result<String>>;
}

#[cfg(test)]
pub(crate) struct StaticTokenSource(pub String);

#[cfg(test)]
impl OAuthTokenSource for StaticTokenSource {
    fn get_access_token(&self) -> BoxFuture<'_, Result<String>> {
        let v = self.0.clone();
        Box::pin(async move { Ok(v) })
    }
}

#[cfg(test)]
impl std::fmt::Debug for StaticTokenSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("StaticTokenSource").finish()
    }
}

#[cfg(test)]
pub(crate) struct FailingTokenSource;

#[cfg(test)]
impl OAuthTokenSource for FailingTokenSource {
    fn get_access_token(&self) -> BoxFuture<'_, Result<String>> {
        Box::pin(async {
            Err(crate::api::error::OAuthError::RefreshFailed(
                "test failure".into(),
            ))
        })
    }
}

#[cfg(test)]
impl std::fmt::Debug for FailingTokenSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("FailingTokenSource").finish()
    }
}
