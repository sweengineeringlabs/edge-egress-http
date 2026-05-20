//! HTTP-specific API types grouped by shared `http_` prefix.

pub mod http_outbound_build_error;
pub mod http_outbound_config;
pub mod http_outbound_config_builder;

pub use http_outbound_build_error::HttpOutboundBuildError;
pub use http_outbound_config::HttpOutboundConfig;
pub use http_outbound_config_builder::HttpOutboundConfigBuilder;
