//! Top-level auth-processing contract.
//!
//! Every auth middleware this crate produces implements [`Processor`].
//! Matches the `service_type = "processor"` metadata in `Cargo.toml`.

use futures::future::BoxFuture;

use crate::api::auth::errors::AuthError;

/// Top-level auth-processing contract. Every auth middleware this crate
/// produces implements it. Matches the `service_type = "processor"` metadata
/// in `Cargo.toml`.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait Processor: Send + Sync + std::fmt::Debug {
    /// Identify this processor in log / trace output.
    fn describe(&self) -> &'static str;

    /// Apply the configured auth policy to an outbound request.
    fn process<'a>(&'a self, req: &'a mut reqwest::Request)
        -> BoxFuture<'a, Result<(), AuthError>>;
}
