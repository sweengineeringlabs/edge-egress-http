//! Observability configuration for the metrics HTTP outbound decorator.

use std::sync::Arc;

use swe_observ_metrics::MetricsProvider;

/// Observability configuration for the metrics-observation HTTP outbound decorator.
pub struct ObservationConfig {
    /// Metrics backend that receives per-call counters and histograms.
    pub provider: Arc<dyn MetricsProvider>,
}
