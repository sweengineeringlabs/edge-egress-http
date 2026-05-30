//! SAF layer — public facade.

mod auth_svc;

pub use crate::api::auth::config::AuthConfig;
pub use crate::api::error::AuthError;
pub use crate::api::types::AuthMiddleware;
pub use auth_svc::AuthSvc;
