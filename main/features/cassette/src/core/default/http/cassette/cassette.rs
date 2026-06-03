//! Default impl of [`HttpCassette`](crate::api::traits::HttpCassette).

use crate::api::traits::HttpCassette;
use crate::api::types::cassette::cassette_config::CassetteConfig;

/// Default HttpCassette implementation. `pub(crate)` — consumers
/// never touch this type directly; they go through `saf::cassette_svc`.
#[derive(Debug)]
pub(crate) struct DefaultHttpCassette {
    config: CassetteConfig,
}

#[cfg_attr(
    not(test),
    expect(dead_code, reason = "SEA core/ impl — exercised only by unit tests")
)]
impl DefaultHttpCassette {
    /// Construct from a resolved config.
    pub(crate) fn new(config: CassetteConfig) -> Self {
        Self { config }
    }

    /// Return the stored config.
    pub(crate) fn config(&self) -> &CassetteConfig {
        &self.config
    }
}

impl HttpCassette for DefaultHttpCassette {
    fn describe(&self) -> &'static str {
        const LABEL: &str = "http-cassette";
        LABEL
    }
    fn config(&self) -> &crate::api::types::cassette::cassette_config::CassetteConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_constructs_and_stores_config() {
        let cfg = CassetteConfig::swe_default().expect("baseline parses");
        let d = DefaultHttpCassette::new(cfg);
        let dbg = format!("{d:?}");
        assert!(dbg.contains("DefaultHttpCassette"), "debug output: {dbg}");
    }

    /// @covers: describe
    #[test]
    fn test_describe_returns_crate_name() {
        let cfg = CassetteConfig::swe_default().expect("baseline parses");
        let d = DefaultHttpCassette::new(cfg);
        assert_eq!(d.describe(), "http-cassette");
    }

    /// @covers: config
    #[test]
    fn test_config_returns_stored_config() {
        let cfg = CassetteConfig::swe_default().expect("baseline parses");
        let mode = cfg.mode.clone();
        let d = DefaultHttpCassette::new(cfg);
        assert_eq!(d.config().mode, mode);
    }
}
