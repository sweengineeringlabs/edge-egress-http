//! HTTP outbound trait — makes outbound HTTP requests.

use futures::future::BoxFuture;

use super::http_egress_result::HttpEgressResult;
use crate::api::value_object::{HttpRequest, HttpResponse, HttpStreamResponse};

/// Makes outbound HTTP requests to external services.
pub trait HttpEgress: Send + Sync {
    fn send(&self, request: HttpRequest) -> BoxFuture<'_, HttpEgressResult<HttpResponse>>;

    /// Send a request and return a lazy byte stream rather than a buffered body.
    ///
    /// Auth, rate-limit, and circuit-breaker middleware all apply to the initial
    /// connection. Retry middleware applies to the connection only — a
    /// partially-consumed stream cannot be transparently retried. If the stream
    /// drops mid-response, the caller must decide whether to reconnect.
    fn send_stream(
        &self,
        request: HttpRequest,
    ) -> BoxFuture<'_, HttpEgressResult<HttpStreamResponse>>;

    fn health_check(&self) -> BoxFuture<'_, HttpEgressResult<()>>;

    fn get(&self, url: &str) -> BoxFuture<'_, HttpEgressResult<HttpResponse>> {
        let req = HttpRequest::get(url.to_string());
        self.send(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_egress_is_object_safe() {
        fn _assert_object_safe(_: &dyn HttpEgress) {}
    }
}
