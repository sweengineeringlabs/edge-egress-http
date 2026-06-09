//! `Validator` — config validation contract.

/// Validates that the contained configuration is well-formed before the
/// middleware is constructed.
pub trait Validator {
    /// Return `Ok(())` if the configuration is valid, or `Err` with a
    /// human-readable description of the first violation found.
    fn validate(&self) -> Result<(), String>;
}
