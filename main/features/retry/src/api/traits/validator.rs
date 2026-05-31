//! `Validator` — configuration validation contract.

/// Configuration validation contract.
pub trait Validator {
    /// Validate the configuration.
    fn validate(&self) -> Result<(), String>;
}
