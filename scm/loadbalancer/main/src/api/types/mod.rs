//! Value objects for the loadbalancer middleware API.

pub mod application_config_builder;

pub mod loadbalancer_layer;
pub use loadbalancer_layer::LoadbalancerLayer;

pub mod loadbalancer_svc;
pub use loadbalancer_svc::LoadbalancerSvc;

// Contract types from the loadbalancer library — re-exported so consumers
// only need to depend on this crate.
pub use swe_edge_loadbalancer::{
    Backend, BackendConfig, BackendHealth, BackendId, BackendPoolInstance, LoadbalancerConfig,
    LoadbalancerError as PoolError, Outcome, Strategy,
};
