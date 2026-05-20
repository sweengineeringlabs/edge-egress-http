//! Interface contract for the default pass-through validator.
//!
//! Consumers that need a `Validator` which always returns `Ok(())` use
//! [`AlwaysValidConfig`] as the api-layer type — the `DefaultValidator`
//! in `core/` provides the concrete implementation.

use crate::api::traits::Validator;

/// A marker config type that always passes validation.
///
/// Used in contexts where validation is structurally required but the value
/// is known to be well-formed at the call site. The `DefaultValidator<T>` in
/// `core/` wraps any `T: Validator` and delegates to it.
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
