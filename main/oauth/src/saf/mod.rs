//! SAF layer — public facade.
//!
//! [`OAuthSvc`] is the sole entry point. Callers supply their
//! own `OAuthTokenSource` implementation; this crate provides the
//! middleware shell that wraps it.

mod oauth_svc;

pub use crate::api::error::{OAuthError, Result};
pub use crate::api::traits::oauth::{OAuthBuilderOps, OAuthTokenSource};
pub use crate::api::types::oauth::{OAuthConfig, OAuthCredentials, OAuthMiddleware, OAuthProvider};
pub use crate::api::types::OAuthBuilder;
pub use crate::api::types::{ApplicationConfigBuilder, OAuthSvc};
