//! Concrete [`Validator`] implementation for [`HttpConfig`].

use crate::api::traits::Validator;
use crate::api::types::HttpConfig;

/// Validates an [`HttpConfig`] value for production use.
pub(crate) struct HttpConfigValidator<'a> {
    config: &'a HttpConfig,
}

impl<'a> HttpConfigValidator<'a> {
    pub(crate) fn new(config: &'a HttpConfig) -> Self {
        Self { config }
    }
}

impl<'a> Validator for HttpConfigValidator<'a> {
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
    fn test_new_creates_validator_from_config_reference() {
        let cfg = HttpConfig::default();
        let v = HttpConfigValidator::new(&cfg);
        // `new` must create a validator that can be called immediately.
        assert!(v.validate().is_ok());
    }

    #[test]
    fn test_http_config_validator_ok_for_valid_config() {
        let cfg = HttpConfig::default();
        let v = HttpConfigValidator::new(&cfg);
        assert!(v.validate().is_ok());
    }

    #[test]
    fn test_http_config_validator_err_for_zero_timeout() {
        let cfg = HttpConfig {
            timeout_secs: 0,
            ..HttpConfig::default()
        };
        let v = HttpConfigValidator::new(&cfg);
        let err = v.validate().unwrap_err();
        assert!(err.contains("timeout_secs"), "got: {err:?}");
    }

    #[test]
    fn test_http_config_validator_err_for_zero_connect_timeout() {
        let cfg = HttpConfig {
            connect_timeout_secs: 0,
            ..HttpConfig::default()
        };
        let v = HttpConfigValidator::new(&cfg);
        let err = v.validate().unwrap_err();
        assert!(err.contains("connect_timeout_secs"), "got: {err:?}");
    }
}
