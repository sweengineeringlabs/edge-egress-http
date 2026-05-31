//! Integration tests for `WsReceiver`.

use futures::stream;
use swe_edge_egress_http_transport::WsReceiver;

#[test]
fn test_ws_receiver_type_empty_stream_is_valid() {
    let _r: WsReceiver = Box::pin(stream::empty());
}
