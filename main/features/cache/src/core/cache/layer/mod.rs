//! Core cache layer implementation — TTL-based HTTP middleware.

mod layer;
mod request_snapshot;
mod ttl_decision;

pub(crate) use layer::{
    extract_etag, reconstruct, snapshot_vary_values_from_snapshot, vary_from_headers,
};
pub(crate) use request_snapshot::RequestSnapshot;
pub(crate) use ttl_decision::TtlDecision;
