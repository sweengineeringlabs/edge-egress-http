//! `Validator` — configuration validation contract.

/// Validation contract for circuit-breaker configuration.
pub trait Validator {
    /// The type being validated.
    type Subject;
    /// The error type returned when validation fails.
    type Error;
    /// Validate `subject`, returning `Ok(())` on success or an error otherwise.
    fn validate(subject: &Self::Subject) -> Result<(), Self::Error>;
}
