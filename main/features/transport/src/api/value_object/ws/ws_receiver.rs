//! WebSocket receive-side stream type (egress).

use std::pin::Pin;

use futures::Stream;

use crate::api::port::http_outbound::HttpOutboundError;
use crate::api::value_object::ws::ws_message::WsMessage;

/// The receive half of a [`WsChannel`](super::ws_channel::WsChannel) (egress).
///
/// Yields [`WsMessage`] frames from the remote WebSocket peer until the
/// connection is closed.
pub type WsReceiver = Pin<Box<dyn Stream<Item = Result<WsMessage, HttpOutboundError>> + Send>>;

#[cfg(test)]
mod tests {
    use futures::stream;

    use super::*;

    /// @covers: WsReceiver
    #[test]
    fn test_ws_receiver_empty_stream_is_valid() {
        let _r: WsReceiver = Box::pin(stream::empty());
    }
}
