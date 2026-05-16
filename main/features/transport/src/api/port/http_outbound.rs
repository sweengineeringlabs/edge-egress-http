//! HTTP outbound trait — makes outbound HTTP requests.

use futures::future::BoxFuture;
use thiserror::Error;

use crate::api::value_object::{HttpRequest, HttpResponse, HttpStreamResponse};

/// Error type for HTTP outbound operations.
#[derive(Debug, Error)]
pub enum HttpOutboundError {
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

/// Result type for HTTP outbound operations.
pub type HttpOutboundResult<T> = Result<T, HttpOutboundError>;

/// Makes outbound HTTP requests to external services.
pub trait HttpOutbound: Send + Sync {
    fn send(&self, request: HttpRequest) -> BoxFuture<'_, HttpOutboundResult<HttpResponse>>;

    /// Send a request and return a lazy byte stream rather than a buffered body.
    ///
    /// Auth, rate-limit, and circuit-breaker middleware all apply to the initial
    /// connection. Retry middleware applies to the connection only — a
    /// partially-consumed stream cannot be transparently retried. If the stream
    /// drops mid-response, the caller must decide whether to reconnect.
    fn send_stream(
        &self,
        request: HttpRequest,
    ) -> BoxFuture<'_, HttpOutboundResult<HttpStreamResponse>>;

    fn health_check(&self) -> BoxFuture<'_, HttpOutboundResult<()>>;

    fn get(&self, url: &str) -> BoxFuture<'_, HttpOutboundResult<HttpResponse>> {
        let req = HttpRequest::get(url.to_string());
        self.send(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_outbound_is_object_safe() {
        fn _assert_object_safe(_: &dyn HttpOutbound) {}
    }
}
