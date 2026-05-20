//! Interface contract for the `DefaultValidator` implementation.
//!
//! The [`DefaultValidator`] type alias names the dyn-safe [`Validator`] trait
//! interface that `DefaultValidator<T>` (in `core/`) implements.

use crate::api::traits::Validator;

/// Dyn-safe alias for the default pass-through validator interface.
pub type DefaultValidator = dyn Validator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_validator_is_object_safe() {
        fn _check(_: &DefaultValidator) {}
    }
}
