//! Primary trait contracts for `swe_edge_egress_breaker`.
//!
//! Rule 153 requires `pub trait Processor` matching `service_type = "processor"`.
//! Rule 155 requires `pub trait Validator` in api/traits.rs.
//! `CircuitBreakerNode` is the per-host state machine contract.

/// Primary trait for this crate (service_type = "processor").
/// Every circuit-breaker middleware produced by this crate implements it.
pub trait Processor: Send + Sync {
    /// Identify this processor in log / trace output.
    ///
    /// Returns the crate's canonical name (e.g. `"swe_edge_egress_breaker"`).
    fn describe(&self) -> &'static str;
}

/// Validation contract for circuit-breaker configuration.
pub trait Validator {
    /// The type being validated.
    type Subject;
    /// The error type returned when validation fails.
    type Error;

    /// Validate `subject`, returning `Ok(())` on success or an error otherwise.
    fn validate(subject: &Self::Subject) -> Result<(), Self::Error>;
}

/// Contract for per-host circuit breaker state machines.
/// Implementors track failure counts and state transitions;
/// the middleware layer holds one node per host.
pub(crate) trait CircuitBreakerNode {
    /// Called BEFORE dispatching a request. Returns whether to
    /// proceed or reject fast; may promote Open → HalfOpen.
    fn admit(
        &mut self,
        config: &crate::api::types::breaker::config::BreakerConfig,
    ) -> crate::api::types::breaker::state::Admission;

    /// Called AFTER dispatching a request that `admit` approved.
    /// Updates internal state based on outcome.
    fn record(
        &mut self,
        config: &crate::api::types::breaker::config::BreakerConfig,
        outcome: crate::api::types::breaker::state::Outcome,
    );
}
