//! The in-memory cached response entry for RFC 7234 caching.

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// One cached response entry — the minimal shape needed to
/// reconstruct a `reqwest::Response`, plus the metadata needed
/// for RFC 7234 `Vary` matching, RFC 7234 `ETag`-based
/// revalidation, and RFC 5861 `stale-while-revalidate`.
#[derive(Clone, Debug)]
pub(crate) struct CachedEntry {
    /// HTTP status code captured at store time.
    pub(crate) status: u16,
    /// Response headers captured at store time (snake-case keys
    /// as returned by `HeaderName::as_str`).
    pub(crate) headers: BTreeMap<String, String>,
    /// Response body bytes, shared via `Arc` so we can hand out
    /// cheap clones to the reconstruction path.
    pub(crate) body: Arc<Vec<u8>>,
    /// Freshness deadline. Serve as-fresh while `Instant::now() <
    /// expires_at`. Serve as-stale-but-reusable during the SWR
    /// window. Must revalidate beyond `expires_at + swr`.
    pub(crate) expires_at: Instant,
    /// `ETag` header from the response, if any. Used to send
    /// `If-None-Match` on revalidation.
    pub(crate) etag: Option<String>,
    /// For each name in the response's `Vary` header, the value
    /// of that REQUEST header at store time. Sorted by header
    /// name (lowercase) so equality checks are order-independent.
    /// Empty when the response had no `Vary`.
    pub(crate) vary_headers: Vec<(String, String)>,
    /// RFC 5861 `stale-while-revalidate` window. `None` when the
    /// upstream did not emit the directive.
    pub(crate) stale_while_revalidate: Option<Duration>,
}
