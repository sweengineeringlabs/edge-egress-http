//! A marker config type that always passes validation.

use crate::api::traits::Validator;

/// A marker config type that always passes validation.
pub struct AlwaysValidConfig;

impl Validator for AlwaysValidConfig {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_valid_config_validate_returns_ok() {
        assert!(AlwaysValidConfig.validate().is_ok());
    }
}
