//! Interface contract for the `DefaultHttpEgress` implementation.
//!
//! The [`DefaultHttpEgress`] type alias names the dyn-safe [`HttpEgress`] trait
//! interface that `DefaultHttpEgress` (in `core/`) implements.

use crate::api::port::HttpEgress;

/// Dyn-safe alias for the default (reqwest-backed) HTTP outbound interface.
///
/// Callers that want a trait object of the default HTTP outbound use
/// `Arc<DefaultHttpEgress>` instead of `Arc<dyn HttpEgress>`.
pub type DefaultHttpEgress = dyn HttpEgress;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_http_egress_is_object_safe() {
        fn _check(_: &DefaultHttpEgress) {}
    }
}
