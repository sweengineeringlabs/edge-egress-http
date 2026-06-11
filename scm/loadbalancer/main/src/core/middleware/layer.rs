//! Impl blocks for [`LoadbalancerLayer`] — constructor +
//! [`reqwest_middleware::Middleware`] impl.

use std::sync::Arc;

use async_trait::async_trait;

use crate::api::types::loadbalancer::loadbalancer_layer::LoadbalancerLayer;
use swe_edge_loadbalancer::{
    build_backend_pool, report_backend_outcome, select_backend, LoadbalancerConfig, Outcome,
};

impl LoadbalancerLayer {
    pub(crate) fn new(
        config: LoadbalancerConfig,
    ) -> Result<Self, crate::api::error::LoadbalancerMiddlewareError> {
        let pool = build_backend_pool(config)?;
        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    /// Rewrite the request URL to use the selected backend's origin while
    /// keeping the original path, query, and fragment.
    fn rewrite_url(
        orig: &reqwest::Url,
        backend_url: &str,
    ) -> Result<reqwest::Url, crate::api::error::LoadbalancerMiddlewareError> {
        let mut base = reqwest::Url::parse(backend_url).map_err(|e| {
            crate::api::error::LoadbalancerMiddlewareError::InvalidBackendUrl(e.to_string())
        })?;
        base.set_path(orig.path());
        base.set_query(orig.query());
        base.set_fragment(orig.fragment());
        Ok(base)
    }
}

#[async_trait]
impl reqwest_middleware::Middleware for LoadbalancerLayer {
    async fn handle(
        &self,
        mut req: reqwest::Request,
        ext: &mut http::Extensions,
        next: reqwest_middleware::Next<'_>,
    ) -> reqwest_middleware::Result<reqwest::Response> {
        let backend = select_backend(&self.pool)
            .map_err(|e| reqwest_middleware::Error::Middleware(anyhow::anyhow!("{e}")))?;

        let new_url = Self::rewrite_url(req.url(), &backend.url)
            .map_err(|e| reqwest_middleware::Error::Middleware(anyhow::anyhow!("{e}")))?;
        *req.url_mut() = new_url;

        let backend_id = backend.id.clone();
        // Expose the selected backend to outer layers (e.g. a circuit-breaker
        // above this in the chain) so they can report pool outcomes keyed to
        // the correct backend.
        ext.insert(backend_id.clone());
        let result = next.run(req, ext).await;

        let outcome = match &result {
            Ok(resp) if resp.status().is_server_error() => Outcome::Failure {
                reason: resp.status().to_string(),
            },
            Ok(_) => Outcome::Success,
            Err(e) => Outcome::Failure {
                reason: e.to_string(),
            },
        };
        report_backend_outcome(&self.pool, &backend_id, outcome);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use swe_edge_loadbalancer::{BackendConfig, Strategy};

    fn two_backend_config() -> LoadbalancerConfig {
        LoadbalancerConfig {
            strategy: Strategy::RoundRobin,
            backends: vec![
                BackendConfig {
                    url: "https://api-1.internal".to_string(),
                    weight: 1,
                },
                BackendConfig {
                    url: "https://api-2.internal".to_string(),
                    weight: 1,
                },
            ],
        }
    }

    /// @covers: new
    #[test]
    fn test_new_builds_layer_from_valid_config() {
        let layer = LoadbalancerLayer::new(two_backend_config());
        assert!(layer.is_ok(), "valid config must build a layer");
    }

    /// @covers: new
    #[test]
    fn test_new_fails_for_empty_backends() {
        let cfg = LoadbalancerConfig {
            strategy: Strategy::RoundRobin,
            backends: vec![],
        };
        let err = LoadbalancerLayer::new(cfg).unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("invalid") || msg.contains("empty") || msg.contains("pool"),
            "{msg}"
        );
    }

    /// @covers: handle
    #[test]
    fn test_loadbalancer_layer_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<LoadbalancerLayer>();
    }

    /// @covers: rewrite_url
    #[test]
    fn test_rewrite_url_keeps_path_and_query() {
        let orig = reqwest::Url::parse("https://api.example.com/v1/users?page=2#top").unwrap();
        let rewritten =
            LoadbalancerLayer::rewrite_url(&orig, "https://api-1.internal:9000").unwrap();
        assert_eq!(rewritten.host_str(), Some("api-1.internal"));
        assert_eq!(rewritten.port(), Some(9000));
        assert_eq!(rewritten.path(), "/v1/users");
        assert_eq!(rewritten.query(), Some("page=2"));
        assert_eq!(rewritten.fragment(), Some("top"));
    }

    /// @covers: rewrite_url
    #[test]
    fn test_rewrite_url_uses_backend_scheme() {
        let orig = reqwest::Url::parse("https://api.example.com/path").unwrap();
        let rewritten = LoadbalancerLayer::rewrite_url(&orig, "http://internal-api").unwrap();
        assert_eq!(rewritten.scheme(), "http");
    }

    /// @covers: rewrite_url
    #[test]
    fn test_rewrite_url_fails_for_invalid_backend_url() {
        let orig = reqwest::Url::parse("https://api.example.com/path").unwrap();
        let err = LoadbalancerLayer::rewrite_url(&orig, "not a url :// !!!").unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("invalid backend URL"), "{msg}");
    }

    /// @covers: new
    #[test]
    fn test_new_constructs_from_single_backend() {
        let cfg = LoadbalancerConfig {
            strategy: Strategy::RoundRobin,
            backends: vec![BackendConfig {
                url: "https://api.test".to_string(),
                weight: 2,
            }],
        };
        let layer = LoadbalancerLayer::new(cfg);
        assert!(layer.is_ok());
    }
}
