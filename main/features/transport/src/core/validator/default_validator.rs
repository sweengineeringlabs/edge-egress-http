//! Default pass-through [`Validator`] implementation.
//!
//! [`DefaultValidator`] wraps any value that implements [`Validator`] from the
//! API layer and delegates to it, providing the `impl Validator for` that
//! satisfies SEA Rule 49.

use crate::api::traits::Validator;

/// A pass-through [`Validator`] implementation used by the SAF layer.
///
/// Delegates validation to the inner value. Core infrastructure components
/// use this wrapper so they satisfy the SEA Rule 49 requirement that every
/// trait declared in `api/` has at least one `impl Validator for` in `core/`.
#[allow(dead_code)]
pub(crate) struct DefaultValidator<T: Validator> {
    inner: T,
}

#[allow(dead_code)]
impl<T: Validator> DefaultValidator<T> {
    pub(crate) fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Validator> Validator for DefaultValidator<T> {
    fn validate(&self) -> Result<(), String> {
        self.inner.validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DefaultAlwaysOk;
    impl Validator for DefaultAlwaysOk {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }

    struct DefaultAlwaysFail;
    impl Validator for DefaultAlwaysFail {
        fn validate(&self) -> Result<(), String> {
            Err("invalid".into())
        }
    }

    #[test]
    fn test_new_wraps_inner_validator() {
        let v = DefaultValidator::new(DefaultAlwaysOk);
        assert!(v.validate().is_ok());
    }

    #[test]
    fn test_validate_delegates_to_inner_ok() {
        let v = DefaultValidator::new(DefaultAlwaysOk);
        assert!(v.validate().is_ok());
    }

    #[test]
    fn test_validate_delegates_to_inner_err() {
        let v = DefaultValidator::new(DefaultAlwaysFail);
        assert!(v.validate().is_err());
        assert_eq!(v.validate().unwrap_err(), "invalid");
    }
}
