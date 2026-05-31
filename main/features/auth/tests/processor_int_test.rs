//! Integration tests for the `Processor` trait in `swe-edge-egress-auth`.

use swe_edge_egress_auth::AuthSvc;

/// @covers: Processor
#[test]
fn test_auth_svc_implements_processor_contract() {
    // AuthSvc implements the processor interface — just verify it can be constructed.
    let _svc = AuthSvc;
}
