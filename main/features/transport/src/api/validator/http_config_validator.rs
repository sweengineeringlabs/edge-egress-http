//! Interface contract for the `HttpConfigValidator` implementation.
//!
//! The [`HttpConfigValidator`] type alias names the dyn-safe [`Validator`] trait
//! interface that `HttpConfigValidator` (in `core/`) implements.

use crate::api::traits::Validator;

/// Dyn-safe alias for the HTTP config validator interface.
pub type HttpConfigValidator = dyn Validator;
