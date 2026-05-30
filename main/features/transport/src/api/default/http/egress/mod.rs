//! Default HTTP egress types — interface contracts.

pub(crate) mod default_http_egress;
pub(crate) mod transport_config;

pub use default_http_egress::DefaultHttpEgress;
pub use transport_config::TransportConfig;
