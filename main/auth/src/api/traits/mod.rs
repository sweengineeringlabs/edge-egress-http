//! Primary trait contracts for `swe_edge_egress_auth`.
//!
//! - [`Processor`] — the top-level auth-processing contract; implemented by
//!   `DefaultHttpAuth`.
//! - [`Validator`] — config / credential validation contract.
//! - [`HttpAuth`] — internal auth-processor contract (re-exported for
//!   layered use within the crate).
//! - [`CredentialResolver`] — resolves abstract credential sources to
//!   concrete secret values.

pub(crate) mod http_auth;
pub mod processor;
pub mod validator;

pub(crate) use crate::api::credential::traits::CredentialResolver;
pub use http_auth::HttpAuth;
pub use processor::Processor;
pub use validator::Validator;

pub mod auth_strategy;
pub use auth_strategy::AuthStrategy;
