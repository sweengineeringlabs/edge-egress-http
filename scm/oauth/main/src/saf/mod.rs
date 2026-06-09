//! SAF layer — public facade.
//!
//! [`OAuthSvc`] is the sole entry point. Callers supply their
//! own `OAuthTokenSource` implementation; this crate provides the
//! middleware shell that wraps it.

mod oauth_svc;

pub use crate::api::refresh::errors::{OAuthError, Result};
pub use crate::api::refresh::traits::{OAuthBuilderOps, OAuthTokenSource};
pub use crate::api::refresh::types::OAuthBuilder;
pub use crate::api::refresh::types::{ApplicationConfigBuilder, OAuthSvc};
pub use crate::api::refresh::types::{
    OAuthConfig, OAuthCredentials, OAuthMiddleware, OAuthProvider,
};
