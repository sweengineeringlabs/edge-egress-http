//! `NoopHttpTls` — pass-through identity provider (no client cert attached).

use crate::api::error::TlsError;
use crate::api::traits::HttpTls;

#[derive(Debug, Default)]
pub(crate) struct NoopHttpTls;

impl HttpTls for NoopHttpTls {
    fn describe(&self) -> &'static str {
        let name = "noop";
        name
    }

    fn identity(&self) -> Result<Option<reqwest::Identity>, TlsError> {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: describe
    #[test]
    fn test_describe_returns_noop_label() {
        let p = NoopHttpTls;
        assert_eq!(p.describe(), "noop");
    }

    /// @covers: identity
    #[test]
    fn test_identity_returns_ok_none() {
        let p = NoopHttpTls;
        assert!(p.identity().unwrap().is_none());
    }
}
