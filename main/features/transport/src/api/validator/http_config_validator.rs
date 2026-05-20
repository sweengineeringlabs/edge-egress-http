//! Interface contract for the `HttpConfigValidator` implementation.
//!
//! The [`HttpConfigValidator`] type alias names the dyn-safe [`Validator`] trait
//! interface that `HttpConfigValidator` (in `core/`) implements.

use crate::api::traits::Validator;

/// Dyn-safe alias for the HTTP config validator interface.
pub type HttpConfigValidator = dyn Validator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_config_validator_is_object_safe() {
        fn _check(_: &HttpConfigValidator) {}
    }
}
