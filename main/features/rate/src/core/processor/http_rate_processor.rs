//! `impl Processor for HttpRateSvc` — satisfies rule 154.

use crate::api::traits::Processor;
use crate::api::types::rate::HttpRateSvc;

impl Processor for HttpRateSvc {
    fn describe(&self) -> &'static str {
        "swe_edge_egress_rate"
    }
}
