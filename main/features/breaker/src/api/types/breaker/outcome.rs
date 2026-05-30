//! `Outcome` — result of a dispatched request as seen by the breaker.

/// Outcome of a dispatched request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Outcome {
    /// The request completed successfully.
    Success,
    /// The request failed or returned a configured failure status.
    Failure,
}
