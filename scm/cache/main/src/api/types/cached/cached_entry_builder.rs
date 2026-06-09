//! `CachedEntryBuilder` — fluent builder for [`CachedEntry`].

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::api::types::cached_entry::CachedEntry;

/// Fluent builder for [`CachedEntry`].
///
/// Required: `status`, `body`, `expires_at`. All others optional.
#[derive(Default)]
pub struct CachedEntryBuilder {
    status: Option<u16>,
    headers: BTreeMap<String, String>,
    body: Option<Arc<Vec<u8>>>,
    expires_at: Option<Instant>,
    etag: Option<String>,
    vary_headers: Vec<(String, String)>,
    stale_while_revalidate: Option<Duration>,
}

impl CachedEntryBuilder {
    /// Create a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the HTTP status code.
    pub fn with_status(mut self, status: u16) -> Self {
        self.status = Some(status);
        self
    }

    /// Set the response headers.
    pub fn with_headers(mut self, headers: BTreeMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    /// Set the response body bytes.
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(Arc::new(body));
        self
    }

    /// Set the freshness deadline.
    pub fn with_expires_at(mut self, expires_at: Instant) -> Self {
        self.expires_at = Some(expires_at);
        self
    }

    /// Set the ETag header value.
    pub fn with_etag(mut self, etag: impl Into<String>) -> Self {
        self.etag = Some(etag.into());
        self
    }

    /// Set the Vary header capture.
    pub fn with_vary_headers(mut self, vary_headers: Vec<(String, String)>) -> Self {
        self.vary_headers = vary_headers;
        self
    }

    /// Set the stale-while-revalidate window.
    pub fn with_stale_while_revalidate(mut self, duration: Duration) -> Self {
        self.stale_while_revalidate = Some(duration);
        self
    }

    /// Consume the builder and produce a [`CachedEntry`].
    ///
    /// Returns `Err` if any required field is missing.
    pub fn build(self) -> Result<CachedEntry, &'static str> {
        Ok(CachedEntry {
            status: self.status.ok_or("status is required")?,
            headers: self.headers,
            body: self.body.ok_or("body is required")?,
            expires_at: self.expires_at.ok_or("expires_at is required")?,
            etag: self.etag,
            vary_headers: self.vary_headers,
            stale_while_revalidate: self.stale_while_revalidate,
        })
    }
}
