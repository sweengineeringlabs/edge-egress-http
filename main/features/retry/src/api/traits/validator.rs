//! `Validator` — configuration validation contract.

/// Configuration validation contract.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait Validator {
    /// Validate the configuration.
    fn validate(&self) -> Result<(), String>;
}
