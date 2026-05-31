//! Constructor impl for [`TlsLayer`].
//! Struct lives in `api::types::tls::layer` per rule 160.
//! `apply_to` is a public method on the struct (also in api::types::tls::layer).

use std::sync::Arc;

use crate::api::traits::HttpTls;
use crate::api::types::TlsLayer;

impl TlsLayer {
    /// Construct from an already-resolved identity provider.
    pub(crate) fn new(provider: Arc<dyn HttpTls>) -> Self {
        Self { provider }
    }
}
