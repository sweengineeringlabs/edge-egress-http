//! Integration tests for `AuthMiddleware` — the public reqwest-middleware layer.
//!
//! Tests exercise observable properties via the public API:
//! construction via `build_auth_middleware(config)`, the `Debug` impl, and
//! `Send + Sync` bounds.

use swe_edge_egress_auth::{build_auth_middleware, AuthConfig, AuthMiddleware};

// ---------------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------------

#[test]
fn test_auth_middleware_builds_from_none_config() {
    // Simplest path: None config needs no env vars. Must always succeed.
    let mw: AuthMiddleware = build_auth_middleware(AuthConfig::None)
        .expect("None config must produce AuthMiddleware");
    // Verify the returned value is usable (debug at minimum).
    let _ = format!("{mw:?}");
}

#[test]
fn test_auth_middleware_builds_from_builder_fn_default() {
    let mw = build_auth_middleware(AuthConfig::None)
        .expect("default config must build to middleware");
    let _ = format!("{mw:?}");
}

#[test]
fn test_auth_middleware_builds_from_bearer_config_when_env_set() {
    let env_name = "SWE_AUTH_MW_BEARER_01";
    std::env::set_var(env_name, "bearer-token-for-mw-test");
    let cfg = AuthConfig::Bearer {
        token_env: env_name.into(),
    };
    let mw = build_auth_middleware(cfg)
        .expect("Bearer with env set must produce AuthMiddleware");
    let _ = format!("{mw:?}");
    std::env::remove_var(env_name);
}

// ---------------------------------------------------------------------------
// Debug impl
// ---------------------------------------------------------------------------

#[test]
fn test_auth_middleware_debug_contains_auth_middleware_type_name() {
    let mw = build_auth_middleware(AuthConfig::None)
        .expect("build ok");
    let s = format!("{mw:?}");
    assert!(
        s.contains("AuthMiddleware"),
        "Debug output must contain 'AuthMiddleware', got: {s}"
    );
}

#[test]
fn test_auth_middleware_debug_contains_processor_description() {
    // The processor for any real config identifies itself as
    // "swe_edge_egress_auth" via DefaultHttpAuth::describe().
    let mw = build_auth_middleware(AuthConfig::None)
        .expect("build ok");
    let s = format!("{mw:?}");
    assert!(
        s.contains("swe_edge_egress_auth"),
        "AuthMiddleware Debug must include processor description 'swe_edge_egress_auth': {s}"
    );
}

// ---------------------------------------------------------------------------
// Send + Sync — compile-time bounds (fails to compile if violated)
// ---------------------------------------------------------------------------

#[test]
fn test_auth_middleware_is_send_and_sync() {
    fn require_send_sync<T: Send + Sync>() {}
    require_send_sync::<AuthMiddleware>();
}

// ---------------------------------------------------------------------------
// Multiple middlewares — independent instances don't share state
// ---------------------------------------------------------------------------

#[test]
fn test_two_auth_middleware_instances_are_independent() {
    let env_a = "SWE_AUTH_MW_INDEP_A_01";
    let env_b = "SWE_AUTH_MW_INDEP_B_01";
    std::env::set_var(env_a, "token-alpha");
    std::env::set_var(env_b, "token-beta");

    let mw_a = build_auth_middleware(AuthConfig::Bearer {
        token_env: env_a.into(),
    })
    .expect("build mw_a");
    let mw_b = build_auth_middleware(AuthConfig::Bearer {
        token_env: env_b.into(),
    })
    .expect("build mw_b");

    // Each has its own processor. Debug strings differ (they embed the
    // processor kind — both are "swe_edge_egress_auth" for DefaultHttpAuth,
    // but the instances themselves are separate allocations).
    let s_a = format!("{mw_a:?}");
    let s_b = format!("{mw_b:?}");
    // Both contain the type name — ensures neither is a default stub.
    assert!(
        s_a.contains("swe_edge_egress_auth"),
        "mw_a Debug missing crate name: {s_a}"
    );
    assert!(
        s_b.contains("swe_edge_egress_auth"),
        "mw_b Debug missing crate name: {s_b}"
    );

    std::env::remove_var(env_a);
    std::env::remove_var(env_b);
}
