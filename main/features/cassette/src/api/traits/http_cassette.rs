//! `HttpCassette` — primary trait for the cassette crate.

/// The cassette crate's primary trait.
pub trait HttpCassette: Send + Sync {
    /// Identify this processor in log / trace output.
    fn describe(&self) -> &'static str;
}
