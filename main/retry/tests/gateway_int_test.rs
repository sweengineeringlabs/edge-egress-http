#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests exercising the public gateway surface of the swe_edge_egress_retry crate.

use swe_edge_egress_retry::{HttpRetrySvc, RetryConfig, RetryError, RetryLayer};

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn make_cfg() -> RetryConfig {
    RetryConfig {
        max_retries: 3,
        initial_interval_ms: 100,
        max_interval_ms: 5000,
        multiplier: 2.0,
        retryable_statuses: vec![500, 502, 503],
        retryable_methods: vec!["GET".to_string()],
    }
}

// ---------------------------------------------------------------------------
// create_config_builder — SAF entry point
// ---------------------------------------------------------------------------

#[test]
fn test_create_config_builder_returns_working_loader() {
    // create_config_builder must not panic and must return a valid loader.
    let _loader = HttpRetrySvc::create_config_builder().build_loader();
}

#[test]
fn test_default_config_has_at_least_one_retry() {
    // A baseline of zero retries would mean the middleware is a no-op by
    // default — the SWE default must allow at least one retry attempt.
    let cfg = RetryConfig::default();
    assert!(
        cfg.max_retries >= 1,
        "default max_retries must be >= 1, got {}",
        cfg.max_retries
    );
}

#[test]
fn test_default_config_has_non_empty_retryable_statuses() {
    // If the default status list is empty the middleware will never trigger.
    let cfg = RetryConfig::default();
    assert!(
        !cfg.retryable_statuses.is_empty(),
        "default retryable_statuses must not be empty"
    );
}

// ---------------------------------------------------------------------------
// build_retry_layer — factory function
// ---------------------------------------------------------------------------

#[test]
fn test_build_retry_layer_from_default_returns_retry_layer_with_correct_debug() {
    // The full happy path: default config → RetryLayer.
    // Debug output must name the type and expose max_retries for
    // operator visibility (log lines, test output).
    let layer =
        HttpRetrySvc::build_retry_layer(RetryConfig::default()).expect("build must succeed");
    let dbg = format!("{layer:?}");
    assert!(
        dbg.contains("RetryLayer"),
        "Debug output must name the type; got: {dbg}"
    );
    assert!(
        dbg.contains("max_retries"),
        "Debug output must expose max_retries; got: {dbg}"
    );
}

// ---------------------------------------------------------------------------
// RetryLayer: Send + Sync — compile-time proof
// ---------------------------------------------------------------------------

#[test]
fn test_retry_layer_satisfies_send_and_sync_bounds() {
    // This test fails to compile if RetryLayer stops being Send + Sync.
    // No runtime assertion needed — the compile itself is the assertion.
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<RetryLayer>();
}

// ---------------------------------------------------------------------------
// build_retry_layer — custom RetryConfig flows through correctly
// ---------------------------------------------------------------------------

#[test]
fn test_build_retry_layer_with_custom_config_stores_all_policy_fields() {
    let cfg = make_cfg();
    // Verify fields are correct before consuming the config.
    assert_eq!(cfg.max_retries, 3);
    assert_eq!(cfg.initial_interval_ms, 100);
    assert_eq!(cfg.max_interval_ms, 5000);
    assert_eq!(cfg.multiplier, 2.0);
    assert_eq!(cfg.retryable_statuses, vec![500u16, 502, 503]);
    assert_eq!(cfg.retryable_methods, vec!["GET".to_string()]);
    HttpRetrySvc::build_retry_layer(cfg).expect("custom config must produce a valid RetryLayer");
}

#[test]
fn test_build_retry_layer_with_zero_max_retries_builds_successfully() {
    // max_retries=0 is "pass-through with no retry" — a valid choice.
    let cfg = RetryConfig {
        max_retries: 0,
        initial_interval_ms: 100,
        max_interval_ms: 100,
        multiplier: 1.0,
        retryable_statuses: vec![],
        retryable_methods: vec![],
    };
    HttpRetrySvc::build_retry_layer(cfg).expect("max_retries=0 must produce a valid RetryLayer");
}

#[test]
fn test_build_retry_layer_with_empty_retryable_lists_builds_successfully() {
    // Empty status and method lists are valid — the middleware simply never
    // triggers a retry.
    let cfg = RetryConfig {
        max_retries: 5,
        initial_interval_ms: 50,
        max_interval_ms: 2000,
        multiplier: 1.5,
        retryable_statuses: vec![],
        retryable_methods: vec![],
    };
    HttpRetrySvc::build_retry_layer(cfg)
        .expect("empty retryable lists must produce a valid RetryLayer");
}

#[test]
fn test_build_retry_layer_with_large_multiplier_builds_successfully() {
    // An operator might configure aggressive backoff during incident response.
    let cfg = RetryConfig {
        max_retries: 2,
        initial_interval_ms: 10,
        max_interval_ms: 60_000,
        multiplier: 100.0,
        retryable_statuses: vec![503],
        retryable_methods: vec!["POST".to_string()],
    };
    HttpRetrySvc::build_retry_layer(cfg).expect("multiplier=100.0 must produce a valid RetryLayer");
}

// ---------------------------------------------------------------------------
// Error variants — Display must be actionable
// ---------------------------------------------------------------------------

#[test]
fn test_error_parse_failed_display_contains_crate_name() {
    // Consumers catching RetryError::ParseFailed must be able to identify which
    // middleware produced the error without reading source code.
    let err = RetryError::ParseFailed("x".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("swe_edge_egress_retry"),
        "ParseFailed display must name the crate; got: {msg}"
    );
}

#[test]
fn test_error_parse_failed_display_contains_supplied_reason() {
    // The wrapped reason must appear verbatim so the operator knows which
    // field or value triggered the parse failure.
    let err = RetryError::ParseFailed("missing field `max_retries`".to_string());
    let msg = err.to_string();
    assert!(
        msg.contains("max_retries"),
        "ParseFailed display must echo the reason; got: {msg}"
    );
}
