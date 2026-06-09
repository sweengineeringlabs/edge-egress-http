//! Public type — the circuit breaker middleware layer.

use std::sync::Arc;

use moka::future::Cache;

use crate::api::types::breaker::breaker_config::BreakerConfig;

/// Circuit breaker middleware. Attach to a
/// `reqwest_middleware::ClientBuilder` via `.with(layer)`.
pub struct BreakerLayer {
    pub(crate) config: Arc<BreakerConfig>,
    /// Per-host state, keyed by the URL's authority
    /// (host:port). `moka::future::Cache` gives us async-safe
    /// concurrent access with background expiration of
    /// long-idle entries.
    pub(crate) state: Cache<String, Arc<tokio::sync::Mutex<crate::core::host::HostBreaker>>>,
    /// Optional loadbalancer pool. When set (requires the `loadbalancer`
    /// feature), the breaker reports circuit-trip and recovery events back
    /// to the pool so that tripped backends are removed from rotation.
    #[cfg(feature = "loadbalancer")]
    pub(crate) pool: Option<Arc<swe_edge_loadbalancer::BackendPoolInstance>>,
}

impl crate::api::traits::BreakerMetrics for BreakerLayer {
    fn failure_threshold(&self) -> u32 {
        self.config.failure_threshold
    }
}

impl std::fmt::Debug for BreakerLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("BreakerLayer");
        d.field("failure_threshold", &self.config.failure_threshold)
            .field(
                "half_open_after_seconds",
                &self.config.half_open_after_seconds,
            )
            .field("reset_after_successes", &self.config.reset_after_successes);
        #[cfg(feature = "loadbalancer")]
        d.field("pool", &self.pool.is_some());
        d.finish()
    }
}
