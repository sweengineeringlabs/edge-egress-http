//! Integration tests for `HttpStreamResponse`.

use std::collections::HashMap;

use futures::stream;
use swe_edge_egress_http_transport::HttpStreamResponse;

#[test]
fn test_http_stream_response_struct_debug_does_not_expose_stream_internals() {
    let resp = HttpStreamResponse {
        status: 200,
        headers: HashMap::new(),
        body: Box::pin(stream::empty()),
    };
    let dbg = format!("{:?}", resp);
    assert!(dbg.contains("200"));
    assert!(dbg.contains("<stream>"));
}
