//! HTTP port traits.
pub mod http;
pub mod http_outbound;

pub use http::HttpStreamOutbound;
pub use http_outbound::{HttpOutbound, HttpOutboundError, HttpOutboundResult};
