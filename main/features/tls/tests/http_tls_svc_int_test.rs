//! Integration tests for `HttpTlsSvc`.
//!
//! Rule 120: `src/api/types/tls/http_tls_svc.rs` requires a corresponding
//! test file.

use swe_edge_egress_tls::{HttpTlsSvc, TlsConfig};

/// @covers: HttpTlsSvc::build_tls_layer
/// Building with `TlsConfig::None` must succeed.
#[test]
fn tls_struct_http_tls_svc_build_tls_layer_none_succeeds_int_test() {
    let result = HttpTlsSvc::build_tls_layer(TlsConfig::None);
    assert!(
        result.is_ok(),
        "HttpTlsSvc::build_tls_layer(None) must succeed; got: {result:?}"
    );
}

/// @covers: HttpTlsSvc::create_config_builder
/// The config builder must carry a non-empty package name.
#[test]
fn tls_struct_http_tls_svc_create_config_builder_has_name_int_test() {
    let builder = HttpTlsSvc::create_config_builder();
    assert!(
        !builder.name().is_empty(),
        "HttpTlsSvc config builder must carry the package name"
    );
}
