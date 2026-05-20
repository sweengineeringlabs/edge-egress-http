//! Interface contract for the `DefaultHttpOutbound` implementation.
//!
//! The [`DefaultHttpOutbound`] type alias names the dyn-safe [`HttpOutbound`] trait
//! interface that `DefaultHttpOutbound` (in `core/`) implements.

use crate::api::port::HttpOutbound;

/// Dyn-safe alias for the default (reqwest-backed) HTTP outbound interface.
///
/// Callers that want a trait object of the default HTTP outbound use
/// `Arc<DefaultHttpOutbound>` instead of `Arc<dyn HttpOutbound>`.
pub type DefaultHttpOutbound = dyn HttpOutbound;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_http_outbound_is_object_safe() {
        fn _check(_: &DefaultHttpOutbound) {}
    }
}
