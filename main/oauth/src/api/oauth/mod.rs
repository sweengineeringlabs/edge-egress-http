//! OAuth API types — grouped by the `oauth` module to satisfy Rule 112/208.

pub mod o_auth_builder;
pub mod o_auth_builder_ops;
pub mod o_auth_config;
pub mod o_auth_credentials;
pub mod o_auth_middleware;
pub mod o_auth_provider;
pub mod o_auth_token_source;

pub use o_auth_builder::OAuthBuilder;
pub use o_auth_builder_ops::OAuthBuilderOps;
pub use o_auth_config::OAuthConfig;
pub use o_auth_credentials::OAuthCredentials;
pub use o_auth_middleware::OAuthMiddleware;
pub use o_auth_provider::OAuthProvider;
pub use o_auth_token_source::OAuthTokenSource;
