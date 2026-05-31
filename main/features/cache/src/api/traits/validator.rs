//! Configuration validation contract for the cache crate.

/// Configuration validation contract.
pub trait Validator: Send + Sync {
    /// Validate the configuration. Returns `Ok(())` when valid.
    fn validate(&self) -> Result<(), String>;
}
