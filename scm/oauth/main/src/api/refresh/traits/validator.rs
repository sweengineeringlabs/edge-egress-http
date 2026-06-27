//! `Validator` — credential validation contract.

use crate::api::refresh::errors::Result;
use crate::api::refresh::types::OAuthCredentials;

/// Validates that an [`OAuthCredentials`] value is usable.
pub trait Validator {
    /// Validate the credentials.
    fn validate(credentials: &OAuthCredentials) -> Result<()>;
}
