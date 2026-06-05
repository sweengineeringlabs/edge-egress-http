//! Result type for HTTP outbound operations.

use crate::api::error::HttpEgressError;

/// Result type for HTTP outbound operations.
pub type HttpEgressResult<T> = Result<T, HttpEgressError>;
