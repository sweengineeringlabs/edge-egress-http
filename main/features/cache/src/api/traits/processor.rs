//! Primary processing trait for the cache crate.

/// Primary processing trait for this crate.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait Processor: Send + Sync {
    /// Identify this processor in log / trace output.
    fn describe(&self) -> &'static str;
}
