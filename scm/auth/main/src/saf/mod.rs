//! SAF layer — public facade.

mod auth_svc;

pub use crate::api::auth::errors::AuthError;
pub use crate::api::auth::types::AuthConfig;
pub use crate::api::auth::types::AuthMiddleware;
pub use crate::api::auth::types::AuthSvc;
pub use crate::api::strategy::traits::AuthStrategy;
pub use crate::api::strategy::types::AwsSigV4StrategyBuilder;
pub use crate::api::strategy::types::AwsSigV4StrategyConfig;
pub use crate::api::strategy::types::AwsSigV4StrategyConfigBuilder;
