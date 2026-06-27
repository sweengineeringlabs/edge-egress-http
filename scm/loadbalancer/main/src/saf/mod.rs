//! SAF layer — public facade.
//!
//! Re-exports public API types and exposes factory functions as methods on
//! `LoadbalancerSvc`. Traits are NOT re-exported from this module (SEA Rule 126).
//! Core types are NOT re-exported directly (SEA Rule 47).

mod loadbalancer_svc;
mod processor_svc;
mod validator_svc;

pub use loadbalancer_svc::{build_loadbalancer_layer, validate_loadbalancer_config};
