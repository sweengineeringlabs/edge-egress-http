//! SAF layer — public facade.
//!
//! [`OAuthSvc`] is the sole entry point. Callers supply their
//! own `OAuthTokenSource` implementation; this crate provides the
//! middleware shell that wraps it.

mod cached_token_svc;
mod o_auth_builder_ops_svc;
mod o_auth_strategy_svc;
mod o_auth_token_source_svc;
mod oauth_svc;
mod processor_svc;
mod time_helper_svc;
mod validator_svc;

pub(crate) use crate::api::refresh::errors::{OAuthError, Result};
pub(crate) use crate::api::refresh::traits::{OAuthBuilderOps, OAuthTokenSource};
pub(crate) use crate::api::refresh::types::OAuthBuilder;
pub(crate) use crate::api::refresh::types::{ApplicationConfigBuilder, OAuthSvc};
pub(crate) use crate::api::refresh::types::{
    OAuthConfig, OAuthCredentials, OAuthMiddleware, OAuthProvider,
};
