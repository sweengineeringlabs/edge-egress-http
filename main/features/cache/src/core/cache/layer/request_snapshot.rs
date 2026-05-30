//! Snapshot of request fields needed across the middleware handle() body.

use reqwest::header::HeaderMap;

/// Snapshot of the subset of request fields we need to re-use
/// across the handle() body (header lookups for Vary, URL +
/// method for the SWR background refresh).
pub(crate) struct RequestSnapshot {
    pub(crate) method: reqwest::Method,
    pub(crate) url: reqwest::Url,
    pub(crate) headers: HeaderMap,
}

impl RequestSnapshot {
    /// Capture the fields needed for Vary matching and SWR refresh.
    pub(crate) fn new(req: &reqwest::Request) -> Self {
        Self {
            method: req.method().clone(),
            url: req.url().clone(),
            headers: req.headers().clone(),
        }
    }
}

#[cfg(test)]
impl RequestSnapshot {
    fn captured_method(&self) -> &reqwest::Method {
        &self.method
    }

    fn captured_url(&self) -> &reqwest::Url {
        &self.url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: RequestSnapshot::new
    #[test]
    fn test_captured_method_returns_request_method() {
        let req = reqwest::Request::new(
            reqwest::Method::POST,
            reqwest::Url::parse("https://example.test/").expect("url"),
        );
        let snap = RequestSnapshot::new(&req);
        assert_eq!(snap.captured_method(), &reqwest::Method::POST);
    }

    /// @covers: RequestSnapshot::new
    #[test]
    fn test_captured_url_returns_request_url() {
        let url = reqwest::Url::parse("https://example.test/path?q=1").expect("url");
        let req = reqwest::Request::new(reqwest::Method::GET, url.clone());
        let snap = RequestSnapshot::new(&req);
        assert_eq!(snap.captured_url().as_str(), url.as_str());
    }
}
