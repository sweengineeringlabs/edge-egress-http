//! Primary trait declarations for `swe_edge_egress_oauth`.

pub mod processor;
pub mod validator;

pub use processor::Processor;
pub use validator::Validator;

pub mod o_auth_builder_ops;
pub mod o_auth_token_source;

pub use o_auth_builder_ops::OAuthBuilderOps;
pub use o_auth_token_source::OAuthTokenSource;
