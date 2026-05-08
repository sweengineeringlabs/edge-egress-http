//! HTTP outbound trait — makes outbound HTTP requests.

use futures::future::BoxFuture;
use thiserror::Error;

use crate::api::value_object::{HttpRequest, HttpResponse, HttpStreamResponse};

/// Error type for HTTP outbound operations.
#[derive(Debug, Error)]
pub enum HttpOutboundError {
    #[error("connection failed: {0}")]
    ConnectionFailed(String),
    #[error("timeout: {0}")]
    Timeout(String),
    #[error("invalid request: {0}")]
    InvalidRequest(String),
    #[error("internal: {0}")]
    Internal(String),
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
    fn send_stream(&self, request: HttpRequest) -> BoxFuture<'_, HttpOutboundResult<HttpStreamResponse>>;

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
