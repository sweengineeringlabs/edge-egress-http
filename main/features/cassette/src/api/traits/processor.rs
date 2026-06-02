//! `Processor` — primary processing trait (service_type = "processor").

/// Primary processing trait for this crate.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait Processor: Send + Sync {
    /// Identify this processor in log / trace output.
    fn describe(&self) -> &'static str;
}
