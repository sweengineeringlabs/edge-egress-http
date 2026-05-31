//! `Error` — type alias for `BreakerError`.

use crate::api::error::breaker_error::BreakerError;

/// Error type alias for `swe_edge_egress_breaker`.
pub type Error = BreakerError;
