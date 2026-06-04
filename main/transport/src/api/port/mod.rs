//! HTTP port traits.
pub(crate) mod http;
pub(crate) mod http_egress;
pub(crate) mod http_egress_error;
pub(crate) mod http_egress_result;

pub use http::HttpStream;
pub use http_egress::HttpEgress;
pub use http_egress_error::HttpEgressError;
pub use http_egress_result::HttpEgressResult;
