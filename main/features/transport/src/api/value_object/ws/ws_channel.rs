//! WebSocket full-duplex channel value object (egress).

use crate::api::value_object::ws::ws_receiver::WsReceiver;
use crate::api::value_object::ws::ws_sender::WsSender;

/// A full-duplex WebSocket channel to a remote service.
///
/// Returned by [`HttpStreamOutbound::connect_websocket`] after the handshake.
/// Use [`sender`] to push frames and [`receiver`] to consume them.
///
/// [`HttpStreamOutbound::connect_websocket`]: crate::api::port::http::http_stream_outbound::HttpStreamOutbound::connect_websocket
pub struct WsChannel {
    /// Send frames to the remote WebSocket peer.
    pub sender: WsSender,
    /// Receive frames from the remote WebSocket peer.
    pub receiver: WsReceiver,
}

#[cfg(test)]
mod tests {
    use futures::stream;
    use tokio::sync::mpsc;

    use super::*;

    #[test]
    fn test_ws_channel_can_be_constructed() {
        let (tx, _rx) = mpsc::unbounded_channel();
        let _ch = WsChannel {
            sender: tx,
            receiver: Box::pin(stream::empty()),
        };
    }
}
