//! `OAuthTokenSource` — public extension point for OAuth token providers.
//!
//! Implement this trait to plug your own credential store into the edge
//! OAuth middleware. The middleware calls `get_access_token()` on every
//! outbound request; your implementation is responsible for proactive
//! refresh and any credential file I/O.

use async_trait::async_trait;

use crate::api::Result;

/// Provides a valid OAuth2 access token on demand.
///
/// Implementations are responsible for:
/// - Loading credentials from their backing store (file, env, secret manager …)
/// - Proactively refreshing the token before it expires
/// - Persisting refreshed credentials back to the store
///
/// The middleware calls this once per request; implementations should cache
/// the current token in memory and only hit the token endpoint when necessary.
#[async_trait]
pub trait OAuthTokenSource: Send + Sync + std::fmt::Debug + 'static {
    /// Return a valid access token. Must not return an expired token.
    async fn get_access_token(&self) -> Result<String>;
}

#[cfg(test)]
pub(crate) struct StaticTokenSource(pub String);

#[cfg(test)]
#[async_trait]
impl OAuthTokenSource for StaticTokenSource {
    async fn get_access_token(&self) -> Result<String> {
        Ok(self.0.clone())
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
#[async_trait]
impl OAuthTokenSource for FailingTokenSource {
    async fn get_access_token(&self) -> Result<String> {
        Err(crate::api::Error::RefreshFailed("test failure".into()))
    }
}

#[cfg(test)]
impl std::fmt::Debug for FailingTokenSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("FailingTokenSource").finish()
    }
}
