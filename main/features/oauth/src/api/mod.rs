//! API layer — public types and extension trait.

pub mod error;
pub mod token_source;
pub mod types;

pub use error::{Error, Result};
pub use token_source::OAuthTokenSource;
pub use types::{OAuthConfig, OAuthCredentials, OAuthProvider};
pub(crate) mod refresh_strategy;
