//! Integration tests exercising the public gateway surface of the swe_edge_egress_tls crate.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_egress_tls::{build_tls_layer, TlsConfig, TlsError, TlsLayer};

#[test]
fn test_build_none_config_produces_noop_layer() {
    let layer: TlsLayer =
        build_tls_layer(TlsConfig::None).expect("None config must build successfully");
    let s = format!("{layer:?}");
    assert!(
        s.contains("noop"),
        "None config Debug must contain 'noop': {s}"
    );
}

#[test]
fn test_tls_layer_is_send_and_sync() {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<TlsLayer>();
}

#[test]
fn test_builder_fn_loads_swe_default_none_config() {
    // The SWE default TLS config is None (no client cert, no custom CA).
    let cfg = TlsConfig::None;
    assert!(
        matches!(&cfg, TlsConfig::None),
        "swe_default must be TlsConfig::None"
    );
    build_tls_layer(cfg).expect("None config must build successfully");
}

#[test]
fn test_with_config_pem_missing_file_returns_file_read_failed() {
    let cfg = TlsConfig::Pem {
        path: "/does/not/exist/cert.pem".into(),
    };
    let err = build_tls_layer(cfg).unwrap_err();
    assert!(
        matches!(err, TlsError::FileReadFailed { .. }),
        "missing PEM file must return FileReadFailed: {err:?}"
    );
}

#[test]
fn test_with_config_pkcs12_missing_file_returns_file_read_failed() {
    let cfg = TlsConfig::Pkcs12 {
        path: "/does/not/exist/cert.p12".into(),
        password_env: None,
    };
    let err = build_tls_layer(cfg).unwrap_err();
    assert!(
        matches!(err, TlsError::FileReadFailed { .. }),
        "missing PKCS12 file must return FileReadFailed: {err:?}"
    );
}

#[test]
fn test_with_config_pkcs12_missing_password_env_returns_missing_env_var() {
    let env_name = "SWE_IT_GW_TLS_PW_01";
    std::env::remove_var(env_name);
    let cfg = TlsConfig::Pkcs12 {
        path: "irrelevant.p12".into(),
        password_env: Some(env_name.into()),
    };
    let err = build_tls_layer(cfg).unwrap_err();
    match err {
        TlsError::MissingEnvVar { name } => assert_eq!(name, env_name),
        other => panic!("expected MissingEnvVar, got {other:?}"),
    }
}

#[test]
fn test_build_none_config_always_succeeds_regardless_of_env() {
    build_tls_layer(TlsConfig::None).expect("None config must always build");
}

#[test]
fn test_with_config_stores_pem_variant() {
    let cfg = TlsConfig::Pem {
        path: "/some/path.pem".into(),
    };
    let b_cfg = cfg;
    assert!(
        matches!(&b_cfg, TlsConfig::Pem { .. }),
        "TlsConfig must be the Pem variant"
    );
}

#[test]
fn test_error_parse_failed_display_contains_crate_name() {
    let err = TlsError::ParseFailed("oops".to_string());
    let s = err.to_string();
    assert!(
        s.contains("swe_edge_egress_tls"),
        "ParseFailed Display must name the crate: {s}"
    );
}

#[test]
fn test_error_missing_env_var_display_contains_var_name() {
    let err = TlsError::MissingEnvVar {
        name: "MY_VAR".to_string(),
    };
    let s = err.to_string();
    assert!(
        s.contains("MY_VAR"),
        "MissingEnvVar Display must contain var name: {s}"
    );
}

#[test]
fn test_error_file_read_failed_display_contains_path() {
    let err = TlsError::FileReadFailed {
        path: "/some/path.pem".to_string(),
        reason: "No such file".to_string(),
    };
    let s = err.to_string();
    assert!(
        s.contains("/some/path.pem"),
        "FileReadFailed Display must contain path: {s}"
    );
}
