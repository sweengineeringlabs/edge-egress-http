//! Integration tests for the MetricsHttpEgress SAF wrapper.

use std::sync::Arc;

use swe_edge_egress_http_transport::{observe_http_egress, plain_http_egress, HttpConfig};
use swe_observ_metrics::{create_local_metrics_backend, MetricsProvider};

/// @covers: observe_http_egress — wraps a plain outbound with metrics observation.
#[test]
fn test_observe_http_egress_wraps_plain_outbound_without_error() {
    let inner = plain_http_egress(HttpConfig::default()).expect("plain_http_egress");
    let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
    let _observed = observe_http_egress(inner, provider);
}

/// @covers: observe_http_egress — two wrappers can share a cloned provider.
#[test]
fn test_observe_http_egress_two_instances_share_provider() {
    let inner_a = plain_http_egress(HttpConfig::default()).expect("first plain_http_egress");
    let inner_b = plain_http_egress(HttpConfig::default()).expect("second plain_http_egress");
    let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
    let _a = observe_http_egress(inner_a, Arc::clone(&provider));
    let _b = observe_http_egress(inner_b, provider);
}
