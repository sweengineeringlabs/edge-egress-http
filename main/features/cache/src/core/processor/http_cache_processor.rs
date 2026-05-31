//! `impl Processor for HttpCacheSvc` — satisfies rule 154.

use crate::api::traits::Processor;
use crate::api::types::HttpCacheSvc;

impl Processor for HttpCacheSvc {
    fn describe(&self) -> &'static str {
        const LABEL: &str = "http-cache";
        LABEL
    }
}
