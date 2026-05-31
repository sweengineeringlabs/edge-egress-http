//! `impl Processor for HttpRetrySvc` — satisfies rule 154.

use crate::api::traits::Processor;
use crate::api::types::retry::HttpRetrySvc;

impl Processor for HttpRetrySvc {
    fn describe(&self) -> &'static str {
        const LABEL: &str = "http-retry";
        LABEL
    }
}
