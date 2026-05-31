//! `Processor` — primary trait for the breaker crate.

/// Primary trait for this crate (service_type = "processor").
pub trait Processor: Send + Sync {
    /// Identify this processor in log / trace output.
    fn describe(&self) -> &'static str;
}

/// Circuit-breaker state access.
pub trait BreakerMetrics: Send + Sync {
    /// Return the failure threshold configured for this breaker.
    fn failure_threshold(&self) -> u32;
}
