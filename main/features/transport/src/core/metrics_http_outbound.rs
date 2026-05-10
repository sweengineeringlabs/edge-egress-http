use std::sync::Arc;
use std::time::Instant;

use futures::future::BoxFuture;
use swe_observ_metrics::MetricsProvider;

use crate::api::port::http_outbound::{HttpOutbound, HttpOutboundResult};
use crate::api::value_object::{HttpRequest, HttpResponse, HttpStreamResponse};

/// Wraps any [`HttpOutbound`]; records per-call latency, request count, and
/// error count using the supplied [`MetricsProvider`].
pub(crate) struct MetricsHttpOutbound {
    inner:    Arc<dyn HttpOutbound>,
    provider: Arc<dyn MetricsProvider>,
}

impl MetricsHttpOutbound {
    pub(crate) fn new(inner: Arc<dyn HttpOutbound>, provider: Arc<dyn MetricsProvider>) -> Self {
        Self { inner, provider }
    }
}

impl HttpOutbound for MetricsHttpOutbound {
    fn send(&self, request: HttpRequest) -> BoxFuture<'_, HttpOutboundResult<HttpResponse>> {
        let provider = Arc::clone(&self.provider);
        let method   = request.method.to_string();
        let fut      = self.inner.send(request);
        Box::pin(async move {
            let start  = Instant::now();
            let result = fut.await;
            let labels = &[("method", method.as_str())];
            provider.record_counter("edge_egress_requests_total", 1.0, labels);
            provider.record_histogram("edge_egress_latency_us", start.elapsed().as_micros() as f64, labels);
            if result.is_err() {
                provider.record_counter("edge_egress_errors_total", 1.0, labels);
            }
            result
        })
    }

    fn send_stream(&self, request: HttpRequest) -> BoxFuture<'_, HttpOutboundResult<HttpStreamResponse>> {
        let provider = Arc::clone(&self.provider);
        let method   = request.method.to_string();
        let fut      = self.inner.send_stream(request);
        Box::pin(async move {
            let start  = Instant::now();
            let result = fut.await;
            let labels = &[("method", method.as_str())];
            provider.record_counter("edge_egress_requests_total", 1.0, labels);
            provider.record_histogram("edge_egress_latency_us", start.elapsed().as_micros() as f64, labels);
            if result.is_err() {
                provider.record_counter("edge_egress_errors_total", 1.0, labels);
            }
            result
        })
    }

    fn health_check(&self) -> BoxFuture<'_, HttpOutboundResult<()>> {
        self.inner.health_check()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use swe_observ_metrics::create_local_metrics_backend;

    fn provider() -> Arc<dyn MetricsProvider> {
        Arc::new(create_local_metrics_backend())
    }

    struct NoopOutbound;
    impl HttpOutbound for NoopOutbound {
        fn send(&self, _: HttpRequest) -> BoxFuture<'_, HttpOutboundResult<HttpResponse>> {
            Box::pin(async { Ok(HttpResponse { status: 200, headers: Default::default(), body: vec![] }) })
        }
        fn send_stream(&self, _: HttpRequest) -> BoxFuture<'_, HttpOutboundResult<HttpStreamResponse>> {
            Box::pin(async {
                let body: futures::stream::BoxStream<'static, Result<bytes::Bytes, crate::api::port::http_outbound::HttpOutboundError>> =
                    Box::pin(futures::stream::empty());
                Ok(HttpStreamResponse { status: 200, headers: Default::default(), body })
            })
        }
        fn health_check(&self) -> BoxFuture<'_, HttpOutboundResult<()>> {
            Box::pin(async { Ok(()) })
        }
    }

    /// @covers: send — records edge_egress_requests_total on success.
    #[tokio::test]
    async fn test_send_records_egress_requests_total_on_success() {
        let p = provider();
        let m = MetricsHttpOutbound::new(Arc::new(NoopOutbound), Arc::clone(&p));
        m.send(HttpRequest::get("/")).await.unwrap();
        let snaps = p.export();
        assert!(snaps.iter().any(|s| s.name == "edge_egress_requests_total" && s.value == 1.0),
            "expected edge_egress_requests_total=1, got {snaps:?}");
    }

    /// @covers: send — records edge_egress_latency_us histogram.
    #[tokio::test]
    async fn test_send_records_egress_latency_histogram() {
        let p = provider();
        let m = MetricsHttpOutbound::new(Arc::new(NoopOutbound), Arc::clone(&p));
        m.send(HttpRequest::get("/")).await.unwrap();
        let snaps = p.export();
        assert!(snaps.iter().any(|s| s.name == "edge_egress_latency_us"),
            "expected edge_egress_latency_us, got {snaps:?}");
    }

    /// @covers: send — records edge_egress_errors_total on failure.
    #[tokio::test]
    async fn test_send_records_egress_errors_total_on_failure() {
        use crate::api::port::http_outbound::HttpOutboundError;
        struct FailOutbound;
        impl HttpOutbound for FailOutbound {
            fn send(&self, _: HttpRequest) -> BoxFuture<'_, HttpOutboundResult<HttpResponse>> {
                Box::pin(async { Err(HttpOutboundError::ConnectionFailed("refused".into())) })
            }
            fn send_stream(&self, _: HttpRequest) -> BoxFuture<'_, HttpOutboundResult<HttpStreamResponse>> {
                Box::pin(async { Err(HttpOutboundError::ConnectionFailed("refused".into())) })
            }
            fn health_check(&self) -> BoxFuture<'_, HttpOutboundResult<()>> {
                Box::pin(async { Ok(()) })
            }
        }
        let p = provider();
        let m = MetricsHttpOutbound::new(Arc::new(FailOutbound), Arc::clone(&p));
        let _ = m.send(HttpRequest::get("/")).await;
        let snaps = p.export();
        assert!(snaps.iter().any(|s| s.name == "edge_egress_errors_total" && s.value == 1.0),
            "expected edge_egress_errors_total=1, got {snaps:?}");
    }
}
