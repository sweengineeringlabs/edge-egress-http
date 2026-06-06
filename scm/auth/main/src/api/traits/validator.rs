//! Configuration / credential validation contract.

use crate::api::error::AuthError;

/// Configuration / credential validation contract. Validates that a
/// configuration block is well-formed before any credential resolution
/// or network I/O takes place.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait Validator: Send + Sync {
    /// Validate the configuration. Returns `Ok(())` when valid; an
    /// `AuthError` describing the violation otherwise.
    fn validate(&self) -> Result<(), AuthError>;
}
