//! Fluent builder for the top-level application configuration section.
//!
//! Maps to `config/application.toml` (the `[application]` table).

/// Fluent builder for the `[application]` configuration table.
///
/// Consumers call [`ApplicationConfigBuilder::new`] and chain setter methods,
/// then call [`build`](Self::build) to obtain the final configuration map.
#[derive(Debug, Default, Clone)]
pub struct ApplicationConfigBuilder {
    name: Option<String>,
    version: Option<String>,
}

impl ApplicationConfigBuilder {
    /// Create a new builder with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the application name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the application version string.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Return the configured name, falling back to an empty string.
    pub fn name(&self) -> &str {
        self.name.as_deref().unwrap_or("")
    }

    /// Return the configured version, falling back to `"0.1.0"`.
    pub fn version(&self) -> &str {
        self.version.as_deref().unwrap_or("0.1.0")
    }

    /// Consume the builder and return `self` (identity — no validation required
    /// at this layer; consumers validate after wiring with domain config).
    pub fn build(self) -> Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: with_name
    #[test]
    fn test_with_name_sets_application_name() {
        let b = ApplicationConfigBuilder::new().with_name("my-service");
        assert_eq!(b.name(), "my-service");
    }

    /// @covers: with_version
    #[test]
    fn test_with_version_sets_version_string() {
        let b = ApplicationConfigBuilder::new().with_version("1.2.3");
        assert_eq!(b.version(), "1.2.3");
    }

    /// @covers: build
    #[test]
    fn test_build_returns_configured_builder() {
        let b = ApplicationConfigBuilder::new()
            .with_name("svc")
            .with_version("0.2.0")
            .build();
        assert_eq!(b.name(), "svc");
        assert_eq!(b.version(), "0.2.0");
    }

    /// @covers: name
    #[test]
    fn test_name_defaults_to_empty_string_when_unset() {
        let b = ApplicationConfigBuilder::new();
        assert_eq!(b.name(), "");
    }

    /// @covers: version
    #[test]
    fn test_version_defaults_to_semver_when_unset() {
        let b = ApplicationConfigBuilder::new();
        assert_eq!(b.version(), "0.1.0");
    }
}
