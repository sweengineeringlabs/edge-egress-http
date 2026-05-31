//! `Processor` — primary token-refresh trait for the OAuth crate.

use futures::future::BoxFuture;

use crate::api::error::Result;

/// Processes an OAuth token refresh cycle.
pub trait Processor: Send + Sync + 'static {
    /// Return a valid access token, refreshing if necessary.
    fn process(&self) -> BoxFuture<'_, Result<String>>;
}
