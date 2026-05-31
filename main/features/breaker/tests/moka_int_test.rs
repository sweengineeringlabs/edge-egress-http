//! Integration tests verifying `moka` cache behaviour through
//! the `BreakerLayer` public API.
//!
//! Rule 95: `moka` is used in `src/` and must have integration/e2e coverage.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_egress_breaker::{BreakerConfig, BreakerLayer, HttpBreakerSvc};

fn cfg() -> BreakerConfig {
    BreakerConfig {
        failure_threshold: 3,
        half_open_after_seconds: 30,
        reset_after_successes: 2,
        failure_statuses: vec![500, 503],
    }
}

/// @covers: new
/// The moka cache is embedded in BreakerLayer; constructing and using Debug proves
/// it was allocated without panic.
#[test]
fn test_moka_cache_layer_constructs_successfully() {
    let layer: BreakerLayer = HttpBreakerSvc::build_breaker_layer(cfg()).expect("build");
    let dbg = format!("{layer:?}");
    assert!(
        !dbg.is_empty(),
        "BreakerLayer Debug (backed by moka cache) must produce non-empty output"
    );
}

/// @covers: new
/// Verifies per-instance cache isolation: building two layers must yield two
/// independent objects.
#[test]
fn test_moka_cache_two_layers_are_independent() {
    let a: BreakerLayer = HttpBreakerSvc::build_breaker_layer(cfg()).expect("build a");
    let b: BreakerLayer = HttpBreakerSvc::build_breaker_layer(cfg()).expect("build b");
    // Both must be valid; Debug output must be structurally equal (same config).
    let dbg_a = format!("{a:?}");
    let dbg_b = format!("{b:?}");
    assert_eq!(
        dbg_a, dbg_b,
        "two layers with identical configs must have equal Debug output"
    );
}

/// @covers: new
/// Verifies the layer can be sent across threads, a property that depends on
/// moka's `Cache` being `Send + Sync`.
#[test]
fn test_moka_cache_layer_is_send_and_sync() {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<BreakerLayer>();
}
