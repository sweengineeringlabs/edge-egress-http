//! `BreakerMetrics` — circuit breaker state inspection contract.

/// Circuit-breaker state access.
pub trait BreakerMetrics: Send + Sync {
    /// Return the failure threshold configured for this breaker.
    fn failure_threshold(&self) -> u32;
}
