//! Streaming HTTP response type.

use std::collections::HashMap;
use std::pin::Pin;

use bytes::Bytes;
use futures::Stream;

use crate::api::port::http_egress_error::HttpEgressError;

/// A streaming HTTP response — status and headers are available immediately;
/// the body arrives as a lazy [`Stream`] of [`Bytes`] chunks.
///
/// Unlike [`HttpResponse`](super::HttpResponse), the body is **not buffered**.
/// Callers drive the stream with [`futures::StreamExt::next`]; the connection
/// stays open until the stream is exhausted or dropped.
///
/// # Retry caveat
///
/// Retry middleware applies to the *connection* only. A partially-consumed
/// stream cannot be rewound and retried transparently. If the stream drops
/// mid-response, the caller is responsible for reconnecting.
pub struct HttpStreamResponse {
    /// HTTP status code.
    pub status: u16,
    /// Response headers (lowercase keys).
    pub headers: HashMap<String, String>,
    /// Lazy byte stream. Drive with `futures::StreamExt::next`.
    pub body: Pin<Box<dyn Stream<Item = Result<Bytes, HttpEgressError>> + Send>>,
}

impl std::fmt::Debug for HttpStreamResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpStreamResponse")
            .field("status", &self.status)
            .field("headers", &self.headers)
            .field("body", &"<stream>")
            .finish()
    }
}
