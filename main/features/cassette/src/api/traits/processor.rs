//! `Processor` — primary processing trait (service_type = "processor").

/// Primary processing trait for this crate.
pub trait Processor: Send + Sync {
    /// Identify this processor in log / trace output.
    fn describe(&self) -> &'static str;
}
