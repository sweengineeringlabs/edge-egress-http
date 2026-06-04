//! `Validator` — configuration validation contract.

/// Validates a cassette configuration before it is used to build a layer.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait Validator {
    /// The type being validated.
    type Subject;
    /// The error returned when validation fails.
    type Error;

    /// Validate `subject` and return `Ok(())` or an error.
    fn validate(subject: &Self::Subject) -> Result<(), Self::Error>;
}
