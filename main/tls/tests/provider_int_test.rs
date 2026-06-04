//! Integration tests for the `Provider` trait contract on `HttpTlsSvc`.

use swe_edge_egress_tls::{HttpTlsSvc, Provider};

/// @covers: Provider — trait is object-safe
#[test]
fn tls_trait_provider_is_object_safe_int_test() {
    fn _assert(_: &dyn Provider) {}
}

/// @covers: Provider::describe — HttpTlsSvc returns expected label
#[test]
fn tls_struct_svc_describe_returns_http_tls_label_int_test() {
    let svc = HttpTlsSvc;
    assert_eq!(svc.describe(), "http-tls");
}
