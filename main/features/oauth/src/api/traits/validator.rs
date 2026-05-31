//! `Validator` — credential validation contract.

use crate::api::error::Result;
use crate::api::oauth_credentials::OAuthCredentials;

/// Validates that an [`OAuthCredentials`] value is usable.
pub trait Validator {
    /// Validate the credentials.
    fn validate(credentials: &OAuthCredentials) -> Result<()>;
}
