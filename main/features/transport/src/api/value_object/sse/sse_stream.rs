//! SSE stream type alias (egress).

use std::pin::Pin;

use futures::Stream;

use crate::api::port::http_egress_error::HttpEgressError;
use crate::api::value_object::sse::sse_event::SseEvent;

/// A lazy stream of [`SseEvent`] items consumed from a remote SSE feed.
///
/// The outbound implementation decodes `text/event-stream` frames from the
/// HTTP response body and emits them as [`SseEvent`] items.
pub type SseStream = Pin<Box<dyn Stream<Item = Result<SseEvent, HttpEgressError>> + Send>>;

#[cfg(test)]
mod tests {
    use futures::stream;

    use super::*;

    #[test]
    fn test_sse_stream_empty_stream_is_valid() {
        let _s: SseStream = Box::pin(stream::empty());
    }
}
