//! HTTP error types grouped by prefix.
pub mod http_egress_build_error;
pub mod http_egress_error;
pub use http_egress_build_error::HttpEgressBuildError;
pub use http_egress_error::HttpEgressError;
