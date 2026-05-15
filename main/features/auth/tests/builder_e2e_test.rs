//! End-to-end tests for the swe_edge_egress_auth SAF builder surface.

use swe_edge_egress_auth::{AuthConfig, AuthMiddleware, ApplicationConfigBuilder};

/// @covers: builder
#[test]
fn test_e2e_builder() {
    let mw: AuthMiddleware = swe_edge_egress_auth::builder()
        .expect("builder() must succeed")
        .build()
        .expect("build() must succeed");
    let s = format!("{mw:?}");
    assert!(
        s.contains("swe_edge_egress_auth"),
        "e2e: middleware Debug must name crate: {s}"
    );
}

/// @covers: ApplicationConfigBuilder::with_config
#[test]
fn test_e2e_with_config() {
    let cfg = AuthConfig::None;
    let b = ApplicationConfigBuilder::with_config(cfg);
    assert!(matches!(b.config(), AuthConfig::None));
    let mw = b.build().expect("None config must build");
    assert!(!format!("{mw:?}").is_empty());
}

/// @covers: ApplicationConfigBuilder::config
#[test]
fn test_e2e_config() {
    let b = ApplicationConfigBuilder::with_config(AuthConfig::None);
    let c = b.config();
    assert!(
        matches!(c, AuthConfig::None),
        "config() must return stored policy"
    );
}

/// @covers: ApplicationConfigBuilder::build
#[test]
fn test_e2e_build() {
    let env = "SWE_E2E_AUTH_BEARER_01";
    std::env::set_var(env, "e2e-token");
    let cfg = AuthConfig::Bearer {
        token_env: env.into(),
    };
    let mw = ApplicationConfigBuilder::with_config(cfg)
        .build()
        .expect("bearer e2e build must succeed when env set");
    assert!(!format!("{mw:?}").is_empty());
    std::env::remove_var(env);
}
