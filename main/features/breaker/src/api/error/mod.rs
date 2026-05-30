//! Error type for the breaker middleware.

/// Error type alias for compatibility.
pub type Error = BreakerError;

/// Errors raised by the breaker middleware.
#[derive(Debug, thiserror::Error)]
pub enum BreakerError {
    /// Config TOML didn't parse as the expected schema.
    #[error("swe_edge_egress_breaker: config parse failed — {0}")]
    ParseFailed(String),

    /// The circuit for `host` is open — the request was rejected without
    /// being sent.  Callers can downcast from `reqwest_middleware::Error`
    /// to inspect this variant and apply a fallback.
    #[error("swe_edge_egress_breaker: circuit open for '{host}' — request rejected")]
    CircuitOpen {
        /// The host key for which the circuit tripped.
        host: String,
    },
}
