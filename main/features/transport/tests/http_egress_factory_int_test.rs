//! Integration tests for the SAF factory functions in `http_egress_factory`.
//!
//! Covers: `plain_http_egress`, `default_http_stream_outbound`, and
//! `validate_http_config` — not exercised by other integration test files.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_egress_http_transport::{HttpConfig, HttpStream, HttpTransportSvc};

// ─── plain_http_egress ──────────────────────────────────────────────────────

/// @covers: plain_http_egress
#[test]
fn test_plain_http_egress_builds_with_default_config() {
    let result = HttpTransportSvc::plain_http_egress(HttpConfig::default());
    assert!(
        result.is_ok(),
        "plain_http_egress must build with default config: {:?}",
        result.err()
    );
}

/// @covers: plain_http_egress
#[test]
fn test_plain_http_egress_builds_with_custom_base_url() {
    let cfg = HttpConfig::with_base_url("https://custom.api.com");
    let result = HttpTransportSvc::plain_http_egress(cfg);
    assert!(
        result.is_ok(),
        "plain_http_egress must build with custom base URL: {:?}",
        result.err()
    );
}

// ─── default_http_stream_outbound ────────────────────────────────────────────

/// @covers: default_http_stream_outbound
#[test]
fn test_default_http_stream_outbound_builds_with_swe_defaults() {
    let result = HttpTransportSvc::default_http_stream_outbound();
    assert!(
        result.is_ok(),
        "default_http_stream_outbound must build: {:?}",
        result.err()
    );
}

/// @covers: default_http_stream_outbound
#[test]
fn test_default_http_stream_outbound_implements_stream_outbound_trait() {
    let outbound = HttpTransportSvc::default_http_stream_outbound().unwrap();
    fn _assert(_: &dyn HttpStream) {}
    _assert(outbound.as_ref());
}

// ─── validate_http_config ─────────────────────────────────────────────────────

/// @covers: validate_http_config
#[test]
fn test_validate_http_config_returns_ok_for_valid_timeout() {
    let cfg = HttpConfig {
        timeout_secs: 30,
        connect_timeout_secs: 10,
        ..HttpConfig::default()
    };
    assert!(HttpTransportSvc::validate_http_config(&cfg).is_ok());
}

/// @covers: validate_http_config
#[test]
fn test_validate_http_config_returns_err_for_zero_timeout() {
    let cfg = HttpConfig {
        timeout_secs: 0,
        ..HttpConfig::default()
    };
    let err = HttpTransportSvc::validate_http_config(&cfg).unwrap_err();
    assert!(
        err.contains("timeout_secs"),
        "error must name the offending field, got: {err:?}"
    );
}
