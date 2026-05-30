//! Primary trait re-export hub for `swe_edge_egress_oauth`.
//!
//! Contains the `Processor` (primary) and `Validator` traits that SEA requires
//! for every non-orchestrator `processor` service-type crate.

use futures::future::BoxFuture;

use crate::api::error::Result;
use crate::api::oauth_credentials::OAuthCredentials;

/// Processes an OAuth token refresh cycle.
///
/// The primary trait for this `processor` service-type crate. Implementations
/// fetch, cache, and return a valid bearer access token on demand.
pub trait Processor: Send + Sync + 'static {
    /// Return a valid access token, refreshing if necessary.
    fn process(&self) -> BoxFuture<'_, Result<String>>;
}

/// Validates that an [`OAuthCredentials`] value is usable before it is
/// returned to callers.
///
/// Required for all non-orchestrator crates in SEA.
pub trait Validator {
    /// Validate the credentials, returning `Ok(())` when they are well-formed
    /// and `Err` with a human-readable message when they are not.
    fn validate(credentials: &OAuthCredentials) -> Result<()>;
}
