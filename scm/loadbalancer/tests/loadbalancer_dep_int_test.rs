#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests verifying direct use of the `swe-edge-loadbalancer` dependency.

use swe_edge_loadbalancer::{
    build_backend_pool, report_backend_outcome, select_backend, BackendConfig, BackendHealth,
    BackendId, LoadbalancerConfig, Outcome, Strategy,
};

fn two_backend_config() -> LoadbalancerConfig {
    LoadbalancerConfig {
        strategy: Strategy::RoundRobin,
        backends: vec![
            BackendConfig {
                url: "https://api-1.internal".to_string(),
                weight: 1,
            },
            BackendConfig {
                url: "https://api-2.internal".to_string(),
                weight: 1,
            },
        ],
    }
}

/// @covers: build_backend_pool — constructs pool from valid config
#[test]
fn test_build_backend_pool_constructs_pool_from_valid_config() {
    let pool = build_backend_pool(two_backend_config()).expect("must build");
    assert_eq!(swe_edge_loadbalancer::pool_backend_count(&pool), 2);
}

/// @covers: select_backend — returns a healthy backend
#[test]
fn test_select_backend_returns_healthy_backend() {
    let pool = build_backend_pool(two_backend_config()).expect("must build");
    let backend = select_backend(&pool).expect("must select");
    assert!(
        !backend.url.is_empty(),
        "selected backend url must not be empty"
    );
    assert_eq!(backend.health, BackendHealth::Healthy);
}

/// @covers: report_backend_outcome — failure transitions backend to degraded
#[test]
fn test_report_backend_outcome_failure_transitions_health() {
    let pool = build_backend_pool(two_backend_config()).expect("must build");
    let backend = select_backend(&pool).expect("must select");
    let id = backend.id.clone();
    report_backend_outcome(
        &pool,
        &id,
        Outcome::Failure {
            reason: "500".to_string(),
        },
    );
    // Pool still has healthy backends (second one), so select succeeds.
    let second = select_backend(&pool).expect("must select after failure report");
    assert_eq!(second.health, BackendHealth::Healthy);
}

/// @covers: report_backend_outcome — success keeps backend healthy
#[test]
fn test_report_backend_outcome_success_keeps_backend_healthy() {
    let pool = build_backend_pool(two_backend_config()).expect("must build");
    let backend = select_backend(&pool).expect("must select");
    let id = backend.id.clone();
    report_backend_outcome(&pool, &id, Outcome::Success);
    let next = select_backend(&pool).expect("must still select after success");
    assert_eq!(next.health, BackendHealth::Healthy);
}

/// @covers: BackendId::new — constructs from URL
#[test]
fn test_backend_id_new_stores_url() {
    let id = BackendId::new("https://api.test");
    assert_eq!(id.as_str(), "https://api.test");
}
