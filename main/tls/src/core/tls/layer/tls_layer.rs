//! Constructor impl for [`TlsLayer`].
//! Struct lives in `api::types::tls_layer` per rule 160.
//! `apply_to` is a public method on the struct (also in api::types::tls_layer).

use std::sync::Arc;

use crate::api::traits::HttpTls;
use crate::api::types::TlsLayer;

impl TlsLayer {
    /// Construct from an already-resolved identity provider.
    pub(crate) fn new(provider: Arc<dyn HttpTls>) -> Self {
        Self { provider }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Minimal `HttpTls` implementation used only in unit tests.
    #[derive(Debug)]
    struct TlsLayerStub;

    impl crate::api::traits::HttpTls for TlsLayerStub {
        fn identity(&self) -> Result<Option<reqwest::Identity>, crate::api::error::TlsError> {
            Ok(None)
        }

        fn describe(&self) -> &'static str {
            "noop"
        }
    }

    /// @covers: new
    /// `TlsLayer::new` must construct a layer that reports the provider's
    /// `describe()` string in its Debug output.
    #[test]
    fn tls_struct_tls_layer_new_embeds_provider_describe_int_test() {
        let layer = TlsLayer::new(Arc::new(TlsLayerStub));
        let dbg = format!("{layer:?}");
        assert!(
            dbg.contains("noop"),
            "TlsLayer::new must embed the provider's describe() output; got: {dbg}"
        );
    }

    /// @covers: new
    /// `apply_to` on a newly-constructed noop layer must return `Ok`.
    #[test]
    fn tls_struct_tls_layer_new_apply_to_none_identity_returns_ok_int_test() {
        let layer = TlsLayer::new(Arc::new(TlsLayerStub));
        let result = layer.apply_to(reqwest::Client::builder());
        assert!(
            result.is_ok(),
            "apply_to with a noop provider must return Ok; got: {result:?}"
        );
    }
}
