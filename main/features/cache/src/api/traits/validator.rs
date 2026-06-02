//! Configuration validation contract for the cache crate.

/// Configuration validation contract.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait Validator: Send + Sync {
    /// Validate the configuration. Returns `Ok(())` when valid.
    fn validate(&self) -> Result<(), String>;
}
