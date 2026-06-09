//! SSE stream type alias (egress).

use std::pin::Pin;

use futures::Stream;

use crate::api::error::http::http_egress_error::HttpEgressError;
use crate::api::types::sse::sse_event::SseEvent;

/// A lazy stream of [`SseEvent`] items consumed from a remote SSE feed.
///
/// The outbound implementation decodes `text/event-stream` frames from the
/// HTTP response body and emits them as [`SseEvent`] items.
pub type SseStream = Pin<Box<dyn Stream<Item = Result<SseEvent, HttpEgressError>> + Send>>;
