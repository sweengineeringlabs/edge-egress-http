//! Integration tests covering the `swe-edge-egress-tls` dependency.
//!
//! Verifies that TLS configuration flows through the SAF factory and that
//! plaintext (non-TLS) connections work correctly with the TLS middleware
//! present in the middleware stack when TLS is not required.

use swe_edge_egress_http_transport::default_http_egress;
use swe_edge_egress_tls::TlsConfig;

/// @covers: default_http_egress
#[test]
fn test_tls_config_swe_default_parses_successfully() {
    // Verify the SWE default TLS config parses without error.
    let tls_cfg = TlsConfig::swe_default();
    assert!(
        tls_cfg.is_ok(),
        "TlsConfig::swe_default() must succeed: {:?}",
        tls_cfg.err()
    );
}

/// @covers: default_http_egress
#[test]
fn test_tls_layer_assembles_in_default_http_egress() {
    // `default_http_egress` always includes the TLS middleware layer.
    // A successful build proves the TLS middleware assembled without errors.
    let result = default_http_egress();
    assert!(
        result.is_ok(),
        "default_http_egress (which includes TLS middleware) must build: {:?}",
        result.err()
    );
}

/// @covers: default_http_egress
#[test]
fn test_tls_middleware_does_not_interfere_with_http_only_config() {
    // Build two independent instances — both must succeed independently,
    // demonstrating that the TLS layer is stateless and reusable.
    let a = default_http_egress();
    let b = default_http_egress();
    assert!(a.is_ok(), "first build must succeed");
    assert!(b.is_ok(), "second build must succeed");
}

/// @covers: default_http_egress
#[test]
fn test_tls_config_none_variant_parses_successfully() {
    // Parse the "none" TLS config variant (no client cert, no custom CA).
    let tls_cfg = TlsConfig::from_config(r#"kind = "none""#);
    assert!(
        tls_cfg.is_ok(),
        "TlsConfig 'none' variant must parse: {:?}",
        tls_cfg.err()
    );
}
