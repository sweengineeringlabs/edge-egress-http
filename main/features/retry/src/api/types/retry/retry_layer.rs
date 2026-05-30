//! Public type — the retry middleware layer consumers plug into
//! `reqwest_middleware::ClientBuilder::with(..)`.
//!
//! Rule 160: public types in api/types/. Impl blocks in
//! `core::retry_layer`.

use std::sync::Arc;

use crate::api::types::retry::retry_config::RetryConfig;

/// Retry middleware layer. Opaque handle — consumers get one
/// from `build_retry_layer(config)` and pass it to
/// `reqwest_middleware::ClientBuilder`.
pub struct RetryLayer {
    pub(crate) config: Arc<RetryConfig>,
}

impl std::fmt::Debug for RetryLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RetryLayer")
            .field("max_retries", &self.config.max_retries)
            .field("initial_interval_ms", &self.config.initial_interval_ms)
            .field("max_interval_ms", &self.config.max_interval_ms)
            .finish()
    }
}
