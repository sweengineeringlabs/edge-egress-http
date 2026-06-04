//! Concrete state variants for a per-host circuit breaker.

use std::time::Instant;

/// Concrete state of a breaker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum State {
    /// Traffic flows normally. On enough consecutive failures,
    /// transitions to Open.
    Closed,
    /// All requests fail fast. After `half_open_after_seconds`
    /// elapses since this moment, the NEXT request promotes to
    /// HalfOpen.
    Open { since: Instant },
    /// A probe request is in flight. Outcome decides next
    /// state: success → count up; failure → back to Open.
    HalfOpen,
}
