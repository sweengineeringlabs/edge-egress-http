//! Integration tests for the MetricsHttpOutbound SAF wrapper.

use std::sync::Arc;

use swe_edge_egress_http_transport::{observe_http_outbound, plain_http_outbound, HttpConfig};
use swe_observ_metrics::{create_local_metrics_backend, MetricsProvider};

/// @covers: observe_http_outbound — wraps a plain outbound with metrics observation.
#[test]
fn test_observe_http_outbound_wraps_plain_outbound_without_error() {
    let inner = plain_http_outbound(HttpConfig::default()).expect("plain_http_outbound");
    let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
    let _observed = observe_http_outbound(inner, provider);
}

/// @covers: observe_http_outbound — two wrappers can share a cloned provider.
#[test]
fn test_observe_http_outbound_two_instances_share_provider() {
    let inner_a = plain_http_outbound(HttpConfig::default()).expect("first plain_http_outbound");
    let inner_b = plain_http_outbound(HttpConfig::default()).expect("second plain_http_outbound");
    let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
    let _a = observe_http_outbound(inner_a, Arc::clone(&provider));
    let _b = observe_http_outbound(inner_b, provider);
}
