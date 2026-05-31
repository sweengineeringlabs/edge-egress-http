//! Impl blocks for [`TlsLayer`] — constructor + the
//! `apply_to(reqwest::ClientBuilder)` augmentation method.
//! Struct lives in `api::types::tls::layer` per rule 160; impls here.

use std::sync::Arc;

use crate::api::error::TlsError;
use crate::api::traits::HttpTls;
use crate::api::types::TlsLayer;

impl TlsLayer {
    /// Construct from an already-resolved identity provider.
    pub(crate) fn new(provider: Arc<dyn HttpTls>) -> Self {
        Self { provider }
    }

    /// Augment `builder` with this layer's client identity.
    /// Returns the builder unchanged when the underlying provider
    /// is the `None` (pass-through) variant.
    pub fn apply_to(
        &self,
        builder: reqwest::ClientBuilder,
    ) -> Result<reqwest::ClientBuilder, TlsError> {
        match self.provider.identity()? {
            Some(identity) => Ok(builder.identity(identity)),
            None => Ok(builder),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct NoopStub;
    impl HttpTls for NoopStub {
        fn describe(&self) -> &'static str {
            let name = "noop-stub";
            name
        }
        fn identity(&self) -> Result<Option<reqwest::Identity>, TlsError> {
            Ok(None)
        }
    }

    /// @covers: apply_to
    #[test]
    fn test_apply_to_with_none_provider_returns_builder_unchanged() {
        let layer = TlsLayer::new(Arc::new(NoopStub));
        // Can't easily diff ClientBuilder state; this test just
        // verifies the call completes without error.
        let _b = layer
            .apply_to(reqwest::Client::builder())
            .expect("noop path does not fail");
    }

    /// @covers: new
    #[test]
    fn test_new_holds_provider() {
        let layer = TlsLayer::new(Arc::new(NoopStub));
        assert_eq!(layer.provider.describe(), "noop-stub");
    }
}
