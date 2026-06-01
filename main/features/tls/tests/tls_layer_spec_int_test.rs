//! Integration tests for the `TlsLayerSpec` marker trait.
//!
//! Rule 120: `src/api/tls/layer/tls_layer_spec.rs` requires a corresponding
//! test file.
//!
//! `TlsLayerSpec` is a marker trait for TLS layer implementations.
//! The concrete `TlsLayer` struct implements this via core/. We test
//! the layer behavior through the public API.

use swe_edge_egress_tls::{HttpTlsSvc, TlsConfig, TlsLayer};

/// @covers: TlsLayerSpec (via TlsLayer)
/// A `TlsLayer` (which implements `TlsLayerSpec`) must be constructible via
/// `HttpTlsSvc::build_tls_layer(TlsConfig::None)`.
#[test]
fn tls_trait_tls_layer_spec_layer_is_constructible_int_test() {
    let _layer: TlsLayer =
        HttpTlsSvc::build_tls_layer(TlsConfig::None).expect("TlsLayer must build");
}

/// @covers: TlsLayerSpec (Send + Sync)
/// The layer satisfying `TlsLayerSpec` must be `Send + Sync`.
#[test]
fn tls_trait_tls_layer_spec_is_send_and_sync_int_test() {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<TlsLayer>();
}
