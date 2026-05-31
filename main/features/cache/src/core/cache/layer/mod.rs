//! Core cache layer implementation — TTL-based HTTP middleware.

mod layer;
mod request_snapshot;
mod ttl_decision;

pub(crate) use request_snapshot::RequestSnapshot;
pub(crate) use ttl_decision::TtlDecision;
