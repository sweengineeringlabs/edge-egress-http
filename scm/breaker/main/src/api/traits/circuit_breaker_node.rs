//! `CircuitBreakerNode` — per-host state machine contract.

/// Contract for per-host circuit breaker state machines.
pub trait CircuitBreakerNode {
    /// Decide whether to admit a new request based on the current breaker state.
    fn admit(
        &mut self,
        config: &crate::api::types::breaker_config::BreakerConfig,
    ) -> crate::api::types::state::Admission;

    /// Record the outcome of a request to update the breaker state.
    fn record(
        &mut self,
        config: &crate::api::types::breaker_config::BreakerConfig,
        outcome: crate::api::types::state::Outcome,
    );
}
