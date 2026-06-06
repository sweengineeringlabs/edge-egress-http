//! Integration tests for `WsChannel`.

use futures::stream;
use tokio::sync::mpsc;

use swe_edge_egress_http_transport::WsChannel;

#[test]
fn test_ws_channel_struct_can_be_constructed() {
    let (tx, _rx) = mpsc::unbounded_channel();
    let _ch = WsChannel {
        sender: tx,
        receiver: Box::pin(stream::empty()),
    };
}
