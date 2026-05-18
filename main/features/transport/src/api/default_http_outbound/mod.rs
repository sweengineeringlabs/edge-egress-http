//! API interface types for the default HTTP outbound implementation.
#[allow(clippy::module_inception)]
pub mod default_http_outbound;
pub mod transport_config;
pub use default_http_outbound::DefaultHttpOutbound;
pub use transport_config::TransportConfig;
