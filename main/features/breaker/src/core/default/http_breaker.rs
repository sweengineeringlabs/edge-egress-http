//! Default impl of [`Processor`](crate::api::traits::Processor).
//!
//! `DefaultHttpBreaker` holds a resolved [`BreakerConfig`] and
//! implements the primary `Processor` trait. Consumers access it
//! through the SAF factory — they never name this type directly.

use crate::api::traits::Processor;
use crate::api::types::breaker::breaker_config::BreakerConfig;

/// Default Processor implementation. `pub(crate)` — consumers
/// never touch this type directly; they go through `saf::breaker_svc`.
#[derive(Debug)]
pub(crate) struct DefaultHttpBreaker {
    /// Resolved circuit-breaker policy. Carried so it can be
    /// inspected by future methods (e.g. config introspection).
    config: BreakerConfig,
}

impl DefaultHttpBreaker {
    /// Construct from a resolved config.
    pub(crate) fn new(config: BreakerConfig) -> Self {
        Self { config }
    }

    /// Return the failure threshold from the config.
    pub(crate) fn failure_threshold(&self) -> u32 {
        self.config.failure_threshold
    }
}

impl Processor for DefaultHttpBreaker {
    fn describe(&self) -> &'static str {
        "swe_edge_egress_breaker"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_constructs_and_stores_config() {
        let cfg = BreakerConfig::default();
        let d = DefaultHttpBreaker::new(cfg);
        let dbg = format!("{d:?}");
        assert!(dbg.contains("DefaultHttpBreaker"), "debug output: {dbg}");
    }

    /// @covers: describe
    #[test]
    fn test_describe_returns_crate_name() {
        let cfg = BreakerConfig::default();
        let d = DefaultHttpBreaker::new(cfg);
        assert_eq!(d.describe(), "swe_edge_egress_breaker");
    }

    /// @covers: failure_threshold
    #[test]
    fn test_failure_threshold_reflects_config() {
        let cfg = BreakerConfig {
            failure_threshold: 7,
            half_open_after_seconds: 10,
            reset_after_successes: 2,
            failure_statuses: vec![500],
        };
        let d = DefaultHttpBreaker::new(cfg);
        assert_eq!(d.failure_threshold(), 7);
    }
}
