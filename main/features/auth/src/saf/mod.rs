//! SAF layer — public facade.

mod builder;

pub use crate::api::auth::config::AuthConfig;
pub use crate::api::error::AuthError;
pub use crate::api::types::AuthMiddleware;
pub use builder::{build_auth_middleware, create_config_builder};
