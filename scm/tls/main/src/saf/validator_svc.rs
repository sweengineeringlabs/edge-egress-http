//! SAF anchor for the `Validator` interface — SEA Rule 218 compliance.

use crate::api::traits::Validator;
use crate::api::types::TlsConfig;

/// Validates a [`TlsConfig`]; returns the validation error message on failure.
pub fn validate_tls_config(config: &TlsConfig) -> Result<(), String> {
    config.validate()
}
