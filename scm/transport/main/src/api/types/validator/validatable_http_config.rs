//! A validatable wrapper around HttpConfig.

use crate::api::traits::Validator;
use crate::api::types::HttpConfig;

/// Wraps an [`HttpConfig`](crate::HttpConfig) to make it validatable via the `Validator` trait.
pub struct ValidatableHttpConfig {
    /// The HTTP configuration to validate.
    pub config: HttpConfig,
}

impl Validator for ValidatableHttpConfig {
    fn validate(&self) -> Result<(), String> {
        if self.config.timeout_secs == 0 {
            return Err("timeout_secs must be greater than 0".into());
        }
        if self.config.connect_timeout_secs == 0 {
            return Err("connect_timeout_secs must be greater than 0".into());
        }
        Ok(())
    }
}
