//! `Processor` — primary trait for the breaker crate.

/// Primary trait for this crate (service_type = "processor").
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait Processor: Send + Sync {
    /// Identify this processor in log / trace output.
    fn describe(&self) -> &'static str;
}
