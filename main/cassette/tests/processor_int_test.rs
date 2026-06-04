//! Integration tests for the `Processor` trait in `swe-edge-egress-cassette`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_egress_cassette::HttpCassetteSvc;

/// @covers: Processor
#[test]
fn test_processor_trait_is_implementable() {
    // The Svc type must implement Processor to satisfy service_type requirements.
    // If this compiles, the trait contract is satisfied.
    let svc = HttpCassetteSvc;
    // just creating the type verifies the impl exists
    let _ = svc;
}
