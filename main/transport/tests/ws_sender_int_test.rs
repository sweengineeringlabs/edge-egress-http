//! Integration tests for `WsSender`.

use tokio::sync::mpsc;

use swe_edge_egress_http_transport::{WsMessage, WsSender};

#[test]
fn test_ws_sender_type_can_be_constructed_from_mpsc_channel() {
    let (tx, _rx) = mpsc::unbounded_channel::<WsMessage>();
    let _: WsSender = tx;
}
