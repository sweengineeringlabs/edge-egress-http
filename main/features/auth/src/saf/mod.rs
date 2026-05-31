//! SAF layer — public facade.

mod auth_svc;

pub use crate::api::auth::auth_config::AuthConfig;
pub use crate::api::error::AuthError;
pub use crate::api::strategy::aws::aws_sig_v4_strategy_builder::AwsSigV4StrategyBuilder;
pub use crate::api::strategy::aws::aws_sig_v4_strategy_builder::AwsSigV4StrategyConfig;
pub use crate::api::types::AuthMiddleware;
pub use crate::api::types::AuthSvc;
