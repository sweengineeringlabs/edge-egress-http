//! `impl Processor for HttpRetrySvc`.

use crate::api::traits::Processor;
use crate::api::types::retry::HttpRetrySvc;

impl Processor for HttpRetrySvc {
    fn describe(&self) -> &'static str {
        const LABEL: &str = "http-retry";
        LABEL
    }
}
