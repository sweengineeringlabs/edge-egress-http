//! Primary processing trait for the cache crate.

/// Primary processing trait for this crate.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "SEA api/ interface anchor — exercised only via tests"
    )
)]
pub trait Processor: Send + Sync {
    /// Identify this processor in log / trace output.
    fn describe(&self) -> &'static str;
}
