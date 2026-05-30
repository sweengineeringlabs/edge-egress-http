//! Primary trait declarations for `swe_edge_egress_cassette`.

/// The cassette crate's primary trait. Every middleware layer
/// produced by this crate implements it.
pub trait HttpCassette: Send + Sync {
    /// Identify this processor in log / trace output.
    fn describe(&self) -> &'static str;
}

/// Validates a cassette configuration before it is used to build a layer.
pub trait Validator {
    /// The type being validated.
    type Subject;
    /// The error returned when validation fails.
    type Error;

    /// Validate `subject` and return `Ok(())` or an error.
    fn validate(subject: &Self::Subject) -> Result<(), Self::Error>;
}

/// Primary processing trait for this crate (service_type = "processor").
pub trait Processor: Send + Sync {
    /// Identify this processor in log / trace output.
    fn describe(&self) -> &'static str;
}
