//! `Validator` — credential validation contract.

use crate::api::error::Result;
use crate::api::oauth::o_auth_credentials::OAuthCredentials;

/// Validates that an [`OAuthCredentials`] value is usable.
#[expect(dead_code, reason = "SEA api/ interface anchor — intentionally unused")]
pub trait Validator {
    /// Validate the credentials.
    fn validate(credentials: &OAuthCredentials) -> Result<()>;
}
