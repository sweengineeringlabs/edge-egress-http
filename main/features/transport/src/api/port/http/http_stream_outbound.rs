//! HTTP streaming outbound port — SSE consumption and WebSocket connections.

use futures::future::BoxFuture;

use crate::api::port::http_outbound::HttpOutboundResult;
use crate::api::value_object::sse::SseStream;
use crate::api::value_object::ws::WsChannel;

/// Makes HTTP transport-level streaming connections to external services.
///
/// # SSE (Server-Sent Events)
/// Opens an HTTP connection and returns a lazy stream of
/// [`SseEvent`](crate::api::value_object::sse::SseEvent) frames parsed from
/// the `text/event-stream` response body.
///
/// # WebSocket
/// Completes the WebSocket handshake and returns a full-duplex
/// [`WsChannel`]. The caller may send and receive frames concurrently;
/// the connection stays open until the channel is dropped.
pub trait HttpStreamOutbound: Send + Sync {
    /// Subscribe to an SSE feed at `url`.
    ///
    /// Returns a lazy stream that yields [`SseEvent`](crate::api::value_object::sse::SseEvent)
    /// frames as they arrive from the remote service.
    fn subscribe_sse(&self, url: &str) -> BoxFuture<'_, HttpOutboundResult<SseStream>>;

    /// Open a WebSocket connection to `url`.
    ///
    /// Returns a [`WsChannel`] after the handshake completes.
    fn connect_websocket(&self, url: &str) -> BoxFuture<'_, HttpOutboundResult<WsChannel>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: HttpStreamOutbound
    #[test]
    fn test_http_stream_outbound_is_object_safe() {
        fn _assert_object_safe(_: &dyn HttpStreamOutbound) {}
    }
}
