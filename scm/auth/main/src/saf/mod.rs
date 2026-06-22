//! SAF layer — public facade.

mod auth;
mod aws_strategy_svc;
mod basic_strategy_svc;
mod bearer_strategy_svc;
mod credential_resolver_svc;
mod env_credential_resolver_svc;
mod header_strategy_svc;
mod helper_svc;
mod http_auth_svc;
mod processor_svc;
mod strategy;
mod validator_svc;

// Re-export SAF implementation functions (for api/ to call)
pub use auth::auth_svc::{build_auth_middleware, create_config_builder};

// Re-export API types via lib.rs only (facade delegation)
pub use crate::api::auth::errors::AuthError;
pub use crate::api::auth::types::AuthConfig;
pub use crate::api::auth::types::AuthMiddleware;
pub use crate::api::auth::types::AuthSvc;
pub use crate::api::strategy::traits::AuthStrategy;
pub use crate::api::strategy::types::AwsSigV4StrategyBuilder;
pub use crate::api::strategy::types::AwsSigV4StrategyConfig;
pub use crate::api::strategy::types::AwsSigV4StrategyConfigBuilder;
