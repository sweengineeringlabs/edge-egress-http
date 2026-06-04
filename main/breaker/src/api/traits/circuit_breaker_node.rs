//! `CircuitBreakerNode` — per-host state machine contract.

/// Contract for per-host circuit breaker state machines.
pub trait CircuitBreakerNode {
    fn admit(
        &mut self,
        config: &crate::api::types::breaker::breaker_config::BreakerConfig,
    ) -> crate::api::types::breaker::state::Admission;

    fn record(
        &mut self,
        config: &crate::api::types::breaker::breaker_config::BreakerConfig,
        outcome: crate::api::types::breaker::state::Outcome,
    );
}
