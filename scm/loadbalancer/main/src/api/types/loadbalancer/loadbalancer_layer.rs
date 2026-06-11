//! `LoadbalancerLayer` — the public middleware type.

use std::sync::Arc;

use swe_edge_loadbalancer::BackendPoolInstance;

/// Load-balancer middleware. Attach to a `reqwest_middleware::ClientBuilder`
/// via `.with(layer)`.
///
/// Constructed via [`crate::LoadbalancerSvc::build_layer`] or the
/// [`crate::build_loadbalancer_layer`] SAF function.
///
/// On each request, selects a healthy backend from the pool and rewrites the
/// request URL (scheme + host + port) to point to that backend while
/// preserving the original path, query, and fragment.
pub struct LoadbalancerLayer {
    pub(crate) pool: Arc<BackendPoolInstance>,
}

impl std::fmt::Debug for LoadbalancerLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoadbalancerLayer")
            .field("pool", &self.pool)
            .finish()
    }
}
