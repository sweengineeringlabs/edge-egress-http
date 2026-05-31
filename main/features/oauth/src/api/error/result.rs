//! `Result` type alias for OAuth operations.

pub use crate::api::error::o_auth_error::OAuthError;

/// Convenience alias for OAuth operations.
pub type Result<T> = std::result::Result<T, OAuthError>;
