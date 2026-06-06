//! `impl Processor for HttpCassetteSvc` — satisfies rule 154.

use crate::api::traits::Processor;
use crate::api::types::http_cassette_svc::HttpCassetteSvc;

impl Processor for HttpCassetteSvc {
    fn describe(&self) -> &'static str {
        const LABEL: &str = "http-cassette";
        LABEL
    }
}
