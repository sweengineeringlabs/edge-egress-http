//! `impl Processor for HttpCassetteSvc` — satisfies rule 154.

use crate::api::traits::Processor;
use crate::api::types::cassette::http_cassette_svc::HttpCassetteSvc;

impl Processor for HttpCassetteSvc {
    fn describe(&self) -> &'static str {
        "swe_edge_egress_cassette"
    }
}
