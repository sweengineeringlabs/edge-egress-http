//! SAF layer — public facade.
//!
//! [`OAuthSvc`] is the sole entry point. Callers supply their
//! own `OAuthTokenSource` implementation; this crate provides the
//! middleware shell that wraps it.

mod oauth_svc;

pub use crate::api::error::{OAuthError, Result};
pub use crate::api::oauth::{
    OAuthBuilder, OAuthBuilderOps, OAuthConfig, OAuthCredentials, OAuthMiddleware, OAuthProvider,
    OAuthTokenSource,
};
pub use crate::api::types::{ApplicationConfigBuilder, OAuthSvc};
