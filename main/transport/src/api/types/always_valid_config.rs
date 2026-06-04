//! A marker config type that always passes validation.

use crate::api::traits::Validator;

/// A marker config type that always passes validation.
pub struct AlwaysValidConfig;

impl Validator for AlwaysValidConfig {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
