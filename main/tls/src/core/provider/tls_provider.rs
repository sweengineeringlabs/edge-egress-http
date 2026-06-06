//! `impl Provider for HttpTlsSvc` — satisfies service_type = "provider".

use crate::api::traits::Provider;
use crate::api::types::HttpTlsSvc;

impl Provider for HttpTlsSvc {
    fn describe(&self) -> &'static str {
        const LABEL: &str = "http-tls";
        LABEL
    }
}
