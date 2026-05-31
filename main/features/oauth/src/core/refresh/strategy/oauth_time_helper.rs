//! `OAuthTimeHelper` — clock abstraction for token expiry checks.

use std::time::{SystemTime, UNIX_EPOCH};

/// Clock helper for token expiry arithmetic.
pub(crate) struct OAuthTimeHelper;

impl OAuthTimeHelper {
    /// Return the current time as Unix-epoch milliseconds.
    pub(crate) fn now_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: now_ms
    #[test]
    fn test_now_ms_returns_nonzero_timestamp() {
        let ms = OAuthTimeHelper::now_ms();
        // Year 2000 in milliseconds = 946_684_800_000.  Any real clock will
        // be far past that; a zero result means the system clock is broken.
        assert!(
            ms > 946_684_800_000,
            "now_ms() must return a plausible Unix-epoch millisecond timestamp, got {ms}"
        );
    }

    /// @covers: now_ms
    #[test]
    fn test_now_ms_is_non_decreasing() {
        let t1 = OAuthTimeHelper::now_ms();
        let t2 = OAuthTimeHelper::now_ms();
        assert!(
            t2 >= t1,
            "second call to now_ms() must not return an earlier timestamp (t1={t1}, t2={t2})"
        );
    }
}
