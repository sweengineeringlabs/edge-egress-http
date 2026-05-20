//! Observability configuration for the metrics HTTP outbound decorator.
//!
//! [`ObservationConfig`] carries the observability settings that
//! `MetricsHttpOutbound` (in `core/`) uses to record per-call metrics.

use std::sync::Arc;

use swe_observ_metrics::MetricsProvider;

/// Observability configuration for the metrics-observation HTTP outbound decorator.
pub struct ObservationConfig {
    /// Metrics backend that receives per-call counters and histograms.
    pub provider: Arc<dyn MetricsProvider>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use swe_observ_metrics::create_local_metrics_backend;

    #[test]
    fn test_observation_config_stores_provider() {
        let provider: Arc<dyn MetricsProvider> = Arc::new(create_local_metrics_backend());
        let cfg = ObservationConfig {
            provider: Arc::clone(&provider),
        };
        let snaps = cfg.provider.export();
        assert!(snaps.is_empty(), "fresh provider must have no metrics");
    }
}
