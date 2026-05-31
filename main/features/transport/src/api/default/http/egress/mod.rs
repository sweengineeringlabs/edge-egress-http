//! Default HTTP egress types — interface contracts.

pub(crate) mod http_egress_object;
pub(crate) mod transport_config;

pub use http_egress_object::HttpEgressObject;
pub use transport_config::TransportConfig;
