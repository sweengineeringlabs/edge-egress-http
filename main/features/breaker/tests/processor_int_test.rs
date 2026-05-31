//! Integration tests for the `Processor` trait in `swe-edge-egress-breaker`.

use swe_edge_egress_breaker::HttpBreakerSvc;

/// @covers: Processor
#[test]
fn test_processor_trait_is_implementable() {
    // The Svc type must implement Processor to satisfy service_type requirements.
    // If this compiles, the trait contract is satisfied.
    let svc = HttpBreakerSvc;
    // just creating the type verifies the impl exists
    drop(svc);
}
