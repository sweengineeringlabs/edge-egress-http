//! `impl Processor for HttpCacheSvc` — satisfies rule 154.

use crate::api::traits::Processor;
use crate::api::types::HttpCacheSvc;

impl Processor for HttpCacheSvc {
    fn describe(&self) -> &'static str {
        const LABEL: &str = "http-cache";
        LABEL
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: describe
    #[test]
    fn cache_struct_http_cache_processor_describe_returns_label_int_test() {
        let svc = HttpCacheSvc;
        assert_eq!(
            svc.describe(),
            "http-cache",
            "Processor::describe must return the crate label"
        );
    }
}
