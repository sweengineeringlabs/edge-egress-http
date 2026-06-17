//! SAF layer — public facade.
//!
//! Re-exports public API types and exposes factory functions as methods on
//! `LoadbalancerSvc`. Traits are NOT re-exported from this module (SEA Rule 126).
//! Core types are NOT re-exported directly (SEA Rule 47).

mod loadbalancer_svc;
mod processor_svc;
mod validator_svc;

// Public types re-exported from api/
pub use crate::api::error::LoadbalancerMiddlewareError;
pub use crate::api::types::{
    Backend, BackendConfig, BackendHealth, BackendId, BackendPoolInstance, LoadbalancerConfig,
    LoadbalancerLayer, LoadbalancerSvc, Outcome, PoolError, Strategy,
};

// SAF standalone functions — all take/return api/ types only
pub use loadbalancer_svc::build_loadbalancer_layer;
pub use loadbalancer_svc::validate_loadbalancer_config;
