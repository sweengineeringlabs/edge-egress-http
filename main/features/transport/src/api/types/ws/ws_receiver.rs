//! WebSocket receive-side stream type (egress).

use std::pin::Pin;

use futures::Stream;

use crate::api::port::http_egress_error::HttpEgressError;
use crate::api::types::ws::ws_message::WsMessage;

/// The receive half of a [`WsChannel`](super::ws_channel::WsChannel) (egress).
///
/// Yields [`WsMessage`] frames from the remote WebSocket peer until the
/// connection is closed.
pub type WsReceiver = Pin<Box<dyn Stream<Item = Result<WsMessage, HttpEgressError>> + Send>>;
