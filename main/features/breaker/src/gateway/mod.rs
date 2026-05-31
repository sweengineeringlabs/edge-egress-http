//! Gateway layer — inbound and outbound integration boundaries.

pub(crate) mod egress;
pub(crate) mod ingress;

pub use crate::api::traits::BreakerMetrics;
pub use crate::saf::*;
