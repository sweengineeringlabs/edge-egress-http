//! Integration tests for the MetricsHttpEgress SAF wrapper.

use std::sync::Arc;

use swe_edge_egress_http_transport::{HttpConfig, HttpTransportSvc};
use swe_observ_metrics::{create_local_metrics_backend, MetricsProvider};

/// @covers: observe_http_egress — wraps a plain outbound with metrics observation.
#[test]
fn test_observe_http_egress_wraps_plain_outbound_without_error() {
    let inner =
        HttpTransportSvc::plain_http_egress(HttpConfig::default()).expect("plain_http_egress");
    let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
    let _observed = HttpTransportSvc::observe_http_egress(inner, provider);
}

/// @covers: observe_http_egress — two wrappers can share a cloned provider.
#[test]
fn test_observe_http_egress_two_instances_share_provider() {
    let inner_a = HttpTransportSvc::plain_http_egress(HttpConfig::default())
        .expect("first plain_http_egress");
    let inner_b = HttpTransportSvc::plain_http_egress(HttpConfig::default())
        .expect("second plain_http_egress");
    let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
    let _a = HttpTransportSvc::observe_http_egress(inner_a, Arc::clone(&provider));
    let _b = HttpTransportSvc::observe_http_egress(inner_b, provider);
}
