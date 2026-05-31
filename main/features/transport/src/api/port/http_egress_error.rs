//! HTTP outbound error type.

use thiserror::Error;

/// Error type for HTTP outbound operations.
#[derive(Debug, Error)]
pub enum HttpEgressError {
    /// Transport-level connection failure.
    #[error("connection failed: {0}")]
    ConnectionFailed(String),
    /// Deadline elapsed before a response was received.
    #[error("timeout: {0}")]
    Timeout(String),
    /// The outbound request was malformed.
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    /// Unexpected client-side error.
    #[error("internal: {0}")]
    Internal(String),
    /// Remote returned HTTP 401 — caller not authenticated.
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    /// Remote returned HTTP 403 — caller lacks permission.
    #[error("forbidden: {0}")]
    Forbidden(String),
    /// Remote returned HTTP 404.
    #[error("not found: {0}")]
    NotFound(String),
    /// Remote returned HTTP 429 — rate limit exceeded.
    #[error("rate limited: {0}")]
    RateLimited(String),
    /// Remote returned HTTP 502 — upstream gateway error.
    #[error("bad gateway: {0}")]
    BadGateway(String),
    /// Remote returned HTTP 503.
    #[error("service unavailable: {0}")]
    ServiceUnavailable(String),
}
