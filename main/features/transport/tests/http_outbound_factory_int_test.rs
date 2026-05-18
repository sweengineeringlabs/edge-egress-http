//! Integration tests for the SAF factory functions in `http_outbound_factory`.
//!
//! Covers: `http_outbound`, `http_outbound_with_auth`, `plain_http_outbound`,
//! and `validate_http_config` which are not tested by other integration test files.

use swe_edge_egress_http_transport::{
    http_outbound, http_outbound_with_auth, plain_http_outbound, validate_http_config, HttpConfig,
    HttpOutboundConfig,
};

// ─── helpers ─────────────────────────────────────────────────────────────────

fn build_outbound_config(base_url: &str) -> HttpOutboundConfig {
    HttpOutboundConfig {
        http: HttpConfig::with_base_url(base_url),
        auth: swe_edge_egress_auth::AuthConfig::None,
        token_source: None,
        retry: swe_edge_egress_retry::builder()
            .expect("retry builder")
            .config()
            .clone(),
        rate: swe_edge_egress_rate::builder()
            .expect("rate builder")
            .config()
            .clone(),
        breaker: swe_edge_egress_breaker::builder()
            .expect("breaker builder")
            .config()
            .clone(),
        cache: swe_edge_egress_cache::builder()
            .expect("cache builder")
            .config()
            .clone(),
        cassette: swe_edge_egress_cassette::CassetteConfig::disabled(),
        cassette_name: "unused".to_string(),
        tls: swe_edge_egress_tls::TlsConfig::None,
    }
}

// ─── http_outbound ────────────────────────────────────────────────────────────

/// @covers: http_outbound
#[test]
fn test_http_outbound_builds_successfully_with_none_auth() {
    let cfg = build_outbound_config("https://api.example.com");
    let result = http_outbound(cfg);
    assert!(
        result.is_ok(),
        "http_outbound must build with None auth: {:?}",
        result.err()
    );
}

/// @covers: http_outbound
#[test]
fn test_http_outbound_builds_two_independent_instances() {
    let a = http_outbound(build_outbound_config("https://a.example.com"));
    let b = http_outbound(build_outbound_config("https://b.example.com"));
    assert!(a.is_ok(), "first http_outbound must build");
    assert!(b.is_ok(), "second http_outbound must build");
}

// ─── http_outbound_with_auth ──────────────────────────────────────────────────

/// @covers: http_outbound_with_auth
#[test]
fn test_http_outbound_with_auth_builds_successfully_with_none_auth() {
    let cfg = HttpConfig::with_base_url("https://api.example.com");
    let auth = swe_edge_egress_auth::AuthConfig::None;
    let result = http_outbound_with_auth(cfg, auth);
    assert!(
        result.is_ok(),
        "http_outbound_with_auth must build: {:?}",
        result.err()
    );
}

/// @covers: http_outbound_with_auth
#[test]
fn test_http_outbound_with_auth_builds_two_independent_instances() {
    let a = http_outbound_with_auth(
        HttpConfig::with_base_url("https://a.example.com"),
        swe_edge_egress_auth::AuthConfig::None,
    );
    let b = http_outbound_with_auth(
        HttpConfig::with_base_url("https://b.example.com"),
        swe_edge_egress_auth::AuthConfig::None,
    );
    assert!(a.is_ok(), "first instance must build");
    assert!(b.is_ok(), "second instance must build");
}

// ─── plain_http_outbound ──────────────────────────────────────────────────────

/// @covers: plain_http_outbound
#[test]
fn test_plain_http_outbound_builds_with_default_config() {
    let result = plain_http_outbound(HttpConfig::default());
    assert!(
        result.is_ok(),
        "plain_http_outbound must build with default config: {:?}",
        result.err()
    );
}

/// @covers: plain_http_outbound
#[test]
fn test_plain_http_outbound_builds_with_custom_base_url() {
    let cfg = HttpConfig::with_base_url("https://custom.api.com");
    let result = plain_http_outbound(cfg);
    assert!(
        result.is_ok(),
        "plain_http_outbound must build with custom base URL: {:?}",
        result.err()
    );
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
    assert!(validate_http_config(&cfg).is_ok());
}

/// @covers: validate_http_config
#[test]
fn test_validate_http_config_returns_err_for_zero_timeout() {
    let cfg = HttpConfig {
        timeout_secs: 0,
        ..HttpConfig::default()
    };
    let err = validate_http_config(&cfg).unwrap_err();
    assert!(
        err.contains("timeout_secs"),
        "error must name the offending field, got: {err:?}"
    );
}
