//! Primary trait and Validator for `swe_edge_egress_cassette`.

/// The cassette crate's primary trait. Every middleware layer
/// produced by this crate implements it.
pub trait HttpCassette: Send + Sync {
    /// Identify this processor in log / trace output.
    ///
    /// Returns the crate's canonical name (e.g. `"swe_edge_egress_cassette"`).
    fn describe(&self) -> &'static str;
}

/// Validates a cassette configuration before it is used to build a layer.
///
/// Implementations in `core/` check that required fields are present and
/// that mode values are in the allowed set.
pub trait Validator {
    /// The type being validated.
    type Subject;
    /// The error returned when validation fails.
    type Error;

    /// Validate `subject` and return `Ok(())` or an error describing what
    /// is wrong.
    fn validate(subject: &Self::Subject) -> Result<(), Self::Error>;
}
