//! HTTP egress error types.

pub mod http_egress_error;
pub use http_egress_error::HttpEgressError;

pub mod http;
pub use http::HttpEgressBuildError;
