//! `impl Provider for HttpTlsSvc` — satisfies service_type = "provider".

use crate::api::traits::Provider;
use crate::api::types::tls::HttpTlsSvc;

impl Provider for HttpTlsSvc {
    fn describe(&self) -> &'static str {
        "swe_edge_egress_tls"
    }
}
