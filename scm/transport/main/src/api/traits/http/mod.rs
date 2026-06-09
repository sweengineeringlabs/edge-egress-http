//! HTTP egress traits grouped by prefix.
pub mod http_egress;
pub mod http_stream;
pub use http_egress::HttpEgress;
pub use http_stream::HttpStream;
