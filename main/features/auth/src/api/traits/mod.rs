//! Primary trait contracts for `swe_edge_egress_auth`.
//!
//! - [`Processor`] — the top-level auth-processing contract; implemented by
//!   `DefaultHttpAuth`.
//! - [`Validator`] — config / credential validation contract.
//! - [`HttpAuth`] — internal auth-processor contract (re-exported for
//!   layered use within the crate).
//! - [`CredentialResolver`] — resolves abstract credential sources to
//!   concrete secret values.

pub(crate) mod credential_resolver;
pub(crate) mod http_auth;

pub(crate) use credential_resolver::CredentialResolver;
pub use http_auth::HttpAuth;

use crate::api::error::AuthError;
use futures::future::BoxFuture;

/// Top-level auth-processing contract. Every auth middleware this crate
/// produces implements it. Matches the `service_type = "processor"` metadata
/// in `Cargo.toml`.
pub trait Processor: Send + Sync + std::fmt::Debug {
    /// Identify this processor in log / trace output.
    fn describe(&self) -> &'static str;

    /// Apply the configured auth policy to an outbound request.
    fn process<'a>(&'a self, req: &'a mut reqwest::Request)
        -> BoxFuture<'a, Result<(), AuthError>>;
}

/// Configuration / credential validation contract. Validates that a
/// configuration block is well-formed before any credential resolution
/// or network I/O takes place.
pub trait Validator: Send + Sync {
    /// Validate the configuration. Returns `Ok(())` when valid; an
    /// `AuthError` describing the violation otherwise.
    fn validate(&self) -> Result<(), AuthError>;
}
