//! WebSocket send-side channel type (egress).

use tokio::sync::mpsc;

use crate::api::types::ws::ws_message::WsMessage;

/// The send half of a [`WsChannel`](super::ws_channel::WsChannel) (egress).
///
/// Push [`WsMessage`] frames to the remote WebSocket peer.
pub type WsSender = mpsc::UnboundedSender<WsMessage>;
