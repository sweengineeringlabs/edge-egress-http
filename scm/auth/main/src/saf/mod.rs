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
pub(crate) use auth::auth_svc::{build_auth_middleware, create_config_builder};
