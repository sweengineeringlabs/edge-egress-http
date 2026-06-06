//! SAF layer — public facade.

mod auth_svc;

pub use crate::api::error::AuthError;
pub use crate::api::strategy::types::AwsSigV4StrategyBuilder;
pub use crate::api::strategy::types::AwsSigV4StrategyConfig;
pub use crate::api::strategy::types::AwsSigV4StrategyConfigBuilder;
pub use crate::api::traits::AuthStrategy;
pub use crate::api::types::AuthConfig;
pub use crate::api::types::AuthMiddleware;
pub use crate::api::types::AuthSvc;
