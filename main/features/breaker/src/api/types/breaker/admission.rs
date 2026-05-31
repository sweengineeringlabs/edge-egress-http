//! `Admission` — decision returned when a request arrives at the circuit breaker.

/// Decision returned when a new request arrives at the circuit breaker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Admission {
    /// Pass through — record the outcome afterward.
    Proceed,
    /// Breaker is open — fail fast without calling upstream.
    RejectOpen,
}
