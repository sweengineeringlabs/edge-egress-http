//! `RateBucketOps` — token consumption contract for rate bucket implementations.

use crate::api::types::rate::rate_config::RateConfig;

/// Contract for token-bucket rate limiters.
pub trait RateBucketOps {
    /// Try to consume one token. Returns `Ok(())` if a token was
    /// available; `Err(wait)` is the minimum delay before a
    /// token will be ready.
    fn try_consume(&mut self, config: &RateConfig) -> Result<(), std::time::Duration>;

    /// Refill tokens based on elapsed time.
    fn refill(&mut self, config: &RateConfig);

    /// Try to acquire one token without waiting.
    ///
    /// Returns `Ok(())` if a token was available and consumed.
    /// Returns `Err(wait)` if the bucket is empty.
    fn try_acquire(&mut self, config: &RateConfig) -> Result<(), std::time::Duration>;
}
