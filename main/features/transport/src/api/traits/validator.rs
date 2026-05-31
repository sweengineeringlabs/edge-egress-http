//! `Validator` — outbound configuration validation contract.

/// Validates an outbound configuration or request value.
pub trait Validator {
    /// Returns `Ok(())` when the value is valid, or a human-readable error.
    fn validate(&self) -> Result<(), String>;
}
