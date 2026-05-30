//! Per-host token bucket.
//!
//! Tokens refill at `tokens_per_second` up to `burst_capacity`.
//! Each request tries to consume one token; if none available,
//! the caller waits until a token is ready (calculated from the
//! refill rate).

use std::time::{Duration, Instant};

use crate::api::traits::RateBucketOps;
use crate::api::types::RateConfig;

/// Token bucket state. Not thread-safe on its own — wrap in a
/// mutex for concurrent use (the middleware does this via moka
/// + tokio::sync::Mutex).
#[derive(Debug)]
pub(crate) struct TokenBucket {
    /// Current token count. Fractional — tokens accumulate
    /// linearly even when refill rate isn't a whole number.
    tokens: f64,
    /// When we last refilled the bucket. Used to compute how
    /// many tokens have accumulated since.
    last_refill: Instant,
}

impl RateBucketOps for TokenBucket {
    fn try_consume(
        &mut self,
        config: &crate::api::types::RateConfig,
    ) -> Result<(), std::time::Duration> {
        self.try_acquire(config)
    }
}

impl TokenBucket {
    /// Construct a full bucket (consumers shouldn't be
    /// artificially throttled on startup).
    pub(crate) fn new(config: &RateConfig) -> Self {
        Self {
            tokens: config.burst_capacity as f64,
            last_refill: Instant::now(),
        }
    }

    /// Refill tokens based on elapsed time since last refill.
    /// Caps at `burst_capacity`.
    pub(crate) fn refill(&mut self, config: &RateConfig) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill);
        let added = elapsed.as_secs_f64() * config.tokens_per_second as f64;
        self.tokens = (self.tokens + added).min(config.burst_capacity as f64);
        self.last_refill = now;
    }

    /// Try to acquire one token without waiting.
    ///
    /// Returns `Ok(())` if a token was available + consumed.
    /// Returns `Err(wait)` if the bucket is empty; `wait` is
    /// the time until one token will be available.
    pub(crate) fn try_acquire(&mut self, config: &RateConfig) -> Result<(), Duration> {
        self.refill(config);
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            Ok(())
        } else {
            let deficit = 1.0 - self.tokens;
            let secs_until_one = deficit / config.tokens_per_second as f64;
            Err(Duration::from_secs_f64(secs_until_one))
        }
    }

    #[cfg(test)]
    pub(crate) fn tokens(&self) -> f64 {
        self.tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> RateConfig {
        RateConfig::from_config(
            r#"
                tokens_per_second = 10
                burst_capacity = 20
                per_host = true
            "#,
        )
        .expect("test config must parse")
    }

    /// @covers: TokenBucket::new
    #[test]
    fn test_new_initialises_to_burst_capacity() {
        let cfg = test_config();
        let b = TokenBucket::new(&cfg);
        assert_eq!(b.tokens(), cfg.burst_capacity as f64);
    }

    /// @covers: TokenBucket::try_acquire
    #[test]
    fn test_try_acquire_consumes_one_token_on_success() {
        let cfg = test_config();
        let mut b = TokenBucket::new(&cfg);
        let before = b.tokens();
        b.try_acquire(&cfg)
            .expect("fresh bucket must yield a token");
        assert!(b.tokens() < before, "one token must be consumed");
    }

    /// @covers: TokenBucket::new
    #[test]
    fn test_full_starts_at_burst_capacity() {
        let cfg = test_config();
        let b = TokenBucket::new(&cfg);
        assert_eq!(b.tokens(), 20.0);
    }

    /// @covers: TokenBucket::try_consume
    #[test]
    fn test_try_consume_succeeds_on_fresh_bucket() {
        let cfg = test_config();
        let mut b = TokenBucket::new(&cfg);
        let result = b.try_consume(&cfg);
        assert!(result.is_ok(), "try_consume must succeed on a fresh bucket");
        assert!(b.tokens() < 20.0, "token count must decrease after consume");
    }

    /// @covers: TokenBucket::try_consume
    #[test]
    fn test_try_consume_returns_wait_on_exhausted_bucket() {
        let cfg = test_config();
        let mut b = TokenBucket::new(&cfg);
        for _ in 0..20 {
            b.try_consume(&cfg).expect("drain must succeed");
        }
        match b.try_consume(&cfg) {
            Err(wait) => assert!(
                wait > Duration::from_millis(0),
                "wait must be positive when bucket exhausted"
            ),
            Ok(_) => panic!("expected Err(wait) on exhausted bucket"),
        }
    }

    /// @covers: TokenBucket::try_acquire
    #[test]
    fn test_acquire_succeeds_when_tokens_available() {
        let cfg = test_config();
        let mut b = TokenBucket::new(&cfg);
        assert!(b.try_acquire(&cfg).is_ok());
        assert!(b.tokens() < 20.0);
    }

    /// @covers: TokenBucket::try_acquire
    #[test]
    fn test_acquire_exhausts_bucket_and_returns_wait() {
        let cfg = test_config();
        let mut b = TokenBucket::new(&cfg);
        for _ in 0..20 {
            assert!(b.try_acquire(&cfg).is_ok());
        }
        match b.try_acquire(&cfg) {
            Err(d) => assert!(d >= Duration::from_millis(90)),
            Ok(_) => panic!("expected wait when bucket exhausted"),
        }
    }

    /// @covers: TokenBucket::refill
    #[test]
    fn test_refill_caps_at_burst_capacity() {
        let cfg = test_config();
        let mut b = TokenBucket::new(&cfg);
        b.last_refill = Instant::now() - Duration::from_secs(100);
        b.try_acquire(&cfg)
            .expect("refilled bucket must yield a token");
        assert!((b.tokens() - 19.0).abs() < 0.001);
    }

    /// @covers: TokenBucket::try_acquire
    #[test]
    fn test_refill_restores_tokens_proportional_to_elapsed_time() {
        let cfg = test_config();
        let mut b = TokenBucket::new(&cfg);
        for _ in 0..20 {
            b.try_acquire(&cfg).expect("drain must succeed");
        }
        b.last_refill = Instant::now() - Duration::from_millis(500);
        b.try_acquire(&cfg)
            .expect("partial refill must yield a token");
        assert!(
            (b.tokens() - 4.0).abs() < 0.1,
            "expected ~4 tokens after partial refill, got {}",
            b.tokens()
        );
    }
}
