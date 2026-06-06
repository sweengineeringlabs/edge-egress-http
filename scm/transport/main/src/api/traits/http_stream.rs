//! HTTP streaming port — SSE consumption and WebSocket connections.

use futures::future::BoxFuture;

use crate::api::types::http_egress_result::HttpEgressResult;
use crate::api::types::sse::SseStream;
use crate::api::types::ws::WsChannel;

/// Makes HTTP transport-level streaming connections to external services.
///
/// # SSE (Server-Sent Events)
/// Opens an HTTP connection and returns a lazy stream of
/// [`SseEvent`](crate::api::types::sse::SseEvent) frames parsed from
/// the `text/event-stream` response body.
///
/// # WebSocket
/// Completes the WebSocket handshake and returns a full-duplex
/// [`WsChannel`]. The caller may send and receive frames concurrently;
/// the connection stays open until the channel is dropped.
pub trait HttpStream: Send + Sync {
    /// Subscribe to an SSE feed at `url`.
    ///
    /// Returns a lazy stream that yields [`SseEvent`](crate::api::types::sse::SseEvent)
    /// frames as they arrive from the remote service.
    fn subscribe_sse(&self, url: &str) -> BoxFuture<'_, HttpEgressResult<SseStream>>;

    /// Open a WebSocket connection to `url`.
    ///
    /// Returns a [`WsChannel`] after the handshake completes.
    fn connect_websocket(&self, url: &str) -> BoxFuture<'_, HttpEgressResult<WsChannel>>;
}
