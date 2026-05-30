//! SAF layer — public facade.
//!
//! [`OAuthSvc`] is the sole entry point. Callers supply their
//! own `OAuthTokenSource` implementation; this crate provides the
//! middleware shell that wraps it.

mod oauth_svc;

pub use crate::api::application_config_builder::ApplicationConfigBuilder;
pub use crate::api::error::{OAuthError, Result};
pub use crate::api::oauth_builder::OAuthBuilder;
pub use crate::api::oauth_builder_ops::OAuthBuilderOps;
pub use crate::api::oauth_config::OAuthConfig;
pub use crate::api::oauth_credentials::OAuthCredentials;
pub use crate::api::oauth_middleware::OAuthMiddleware;
pub use crate::api::oauth_provider::OAuthProvider;
pub use crate::api::oauth_token_source::OAuthTokenSource;
pub use oauth_svc::OAuthSvc;
