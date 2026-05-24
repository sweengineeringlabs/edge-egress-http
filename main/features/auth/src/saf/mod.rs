//! SAF layer — public facade.

mod builder;

pub use crate::api::auth_config::AuthConfig;
pub use crate::api::auth_middleware::AuthMiddleware;
pub use crate::api::error::Error;
pub use builder::{build_auth_middleware, create_config_builder};
