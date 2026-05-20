//! Interface contract for the `MetricsHttpOutbound` decorator.
//!
//! The [`MetricsHttpOutbound`] type alias names the dyn-safe [`HttpOutbound`] trait
//! interface that `MetricsHttpOutbound` (in `core/`) implements.

use crate::api::port::HttpOutbound;

/// Dyn-safe alias for the metrics-observation HTTP outbound interface.
pub type MetricsHttpOutbound = dyn HttpOutbound;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_http_outbound_is_object_safe() {
        fn _check(_: &MetricsHttpOutbound) {}
    }
}
