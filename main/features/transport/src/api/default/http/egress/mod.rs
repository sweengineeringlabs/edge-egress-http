//! Default HTTP egress types — interface contracts.

pub(crate) mod egress_object;
pub(crate) mod egress_spec;
pub(crate) mod transport_config;

pub use egress_object::HttpEgressObject;
pub use egress_spec::HttpEgressSpec;
pub use transport_config::TransportConfig;
