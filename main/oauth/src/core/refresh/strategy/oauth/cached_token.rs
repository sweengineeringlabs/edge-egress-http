//! `CachedToken` — in-memory cached OAuth token with expiry.

/// Milliseconds before token expiry to proactively refresh.
pub(super) const REFRESH_WINDOW_MS: u64 = 60_000;

/// Cached access token with its expiry time.
pub(super) struct CachedToken {
    pub(super) value: String,
    /// Unix-epoch milliseconds when the token expires.
    pub(super) expires_at_ms: u64,
}
