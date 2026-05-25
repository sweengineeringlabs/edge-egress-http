//! API layer — public types and extension traits.

pub mod error;
pub mod traits;
pub mod types;

pub use error::{OAuthError, Result};
pub use traits::token_source::OAuthTokenSource;
pub use traits::OAuthBuilderOps;
pub use types::{OAuthConfig, OAuthCredentials, OAuthProvider};
pub(crate) mod refresh_strategy;
