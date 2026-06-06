//! `CircuitBreakerNode` — per-host state machine contract.

/// Contract for per-host circuit breaker state machines.
pub trait CircuitBreakerNode {
    fn admit(
        &mut self,
        config: &crate::api::types::breaker_config::BreakerConfig,
    ) -> crate::api::types::state::Admission;

    fn record(
        &mut self,
        config: &crate::api::types::breaker_config::BreakerConfig,
        outcome: crate::api::types::state::Outcome,
    );
}
