//! `Validator` — credential validation contract.

use crate::api::refresh::errors::Result;
use crate::api::refresh::types::OAuthCredentials;

/// Validates that an [`OAuthCredentials`] value is usable.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "SEA api/ interface anchor — exercised only via tests"
    )
)]
pub trait Validator {
    /// Validate the credentials.
    fn validate(credentials: &OAuthCredentials) -> Result<()>;
}
