//! `Processor` — primary processing trait for the retry crate.

/// Primary processing trait (service_type = "processor").
///
/// Implemented by retry middleware components to identify themselves
/// in log and trace output.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait Processor: Send + Sync {
    /// Identify this processor unit in log / trace output.
    fn describe(&self) -> &'static str;
}
