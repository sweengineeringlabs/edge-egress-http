//! A validatable wrapper around HttpConfig.

use crate::api::traits::Validator;
use crate::api::types::HttpConfig;

/// Wraps an [`HttpConfig`] to make it validatable via the [`Validator`] trait.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validatable_http_config_ok_for_defaults() {
        let v = ValidatableHttpConfig {
            config: HttpConfig::default(),
        };
        assert!(v.validate().is_ok());
    }

    #[test]
    fn test_validatable_http_config_err_for_zero_timeout() {
        let v = ValidatableHttpConfig {
            config: HttpConfig {
                timeout_secs: 0,
                ..HttpConfig::default()
            },
        };
        let err = v.validate().unwrap_err();
        assert!(err.contains("timeout_secs"), "got: {err:?}");
    }
}
