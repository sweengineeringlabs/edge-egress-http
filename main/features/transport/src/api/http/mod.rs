//! HTTP-specific API types grouped by shared `http_` prefix.

pub(crate) mod http_egress_build_error;
pub(crate) mod http_egress_config;
pub(crate) mod http_egress_config_builder;

pub use http_egress_build_error::HttpEgressBuildError;
pub use http_egress_config::HttpEgressConfig;
pub use http_egress_config_builder::HttpEgressConfigBuilder;
