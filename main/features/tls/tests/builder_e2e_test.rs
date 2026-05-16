//! End-to-end tests for the swe_edge_egress_tls SAF builder surface.

use swe_edge_egress_tls::{ApplicationConfigBuilder, TlsConfig, TlsLayer};

/// @covers: builder
#[test]
fn test_e2e_builder() {
    let layer: TlsLayer = swe_edge_egress_tls::builder()
        .expect("builder() must succeed")
        .build()
        .expect("build() must succeed");
    assert!(
        format!("{layer:?}").contains("noop"),
        "e2e: None config must produce noop layer"
    );
}

/// @covers: ApplicationConfigBuilder::with_config
#[test]
fn test_e2e_with_config() {
    let b = ApplicationConfigBuilder::with_config(TlsConfig::None);
    assert!(matches!(b.config(), TlsConfig::None));
    b.build().expect("e2e with_config None build must succeed");
}

/// @covers: ApplicationConfigBuilder::config
#[test]
fn test_e2e_config() {
    let cfg = TlsConfig::Pem {
        path: "/some/cert.pem".into(),
    };
    let b = ApplicationConfigBuilder::with_config(cfg);
    assert!(
        matches!(b.config(), TlsConfig::Pem { .. }),
        "config() must return Pem variant"
    );
}

/// @covers: ApplicationConfigBuilder::build
#[test]
fn test_e2e_build() {
    let layer = ApplicationConfigBuilder::with_config(TlsConfig::None)
        .build()
        .expect("e2e build with None must always succeed");
    assert!(!format!("{layer:?}").is_empty());
}
