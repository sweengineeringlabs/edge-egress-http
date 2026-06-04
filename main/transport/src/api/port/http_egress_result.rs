//! Result type for HTTP outbound operations.

use super::http_egress_error::HttpEgressError;

/// Result type for HTTP outbound operations.
pub type HttpEgressResult<T> = Result<T, HttpEgressError>;
