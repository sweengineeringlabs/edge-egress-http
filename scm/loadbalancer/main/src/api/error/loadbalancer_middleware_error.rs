//! `LoadbalancerMiddlewareError` — errors from the loadbalancer middleware.

/// Errors produced by the loadbalancer middleware during config validation
/// or request dispatch.
#[derive(Debug, thiserror::Error)]
pub enum LoadbalancerMiddlewareError {
    /// Config validation failed.
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),
    /// The backend pool could not find a healthy backend.
    #[error("pool error: {0}")]
    PoolError(#[from] swe_edge_loadbalancer::LoadbalancerError),
    /// A backend URL was not a valid URL.
    #[error("invalid backend URL: {0}")]
    InvalidBackendUrl(String),
}
