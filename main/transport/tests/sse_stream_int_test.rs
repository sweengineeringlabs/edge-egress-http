//! Integration tests for `SseStream`.

use futures::stream;
use swe_edge_egress_http_transport::SseStream;

#[test]
fn test_sse_stream_type_empty_stream_is_valid() {
    let _s: SseStream = Box::pin(stream::empty());
}
