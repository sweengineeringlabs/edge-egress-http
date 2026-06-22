//! [`FileCredentialResolver`] — resolves credentials from files and environment variables.

use std::path::PathBuf;

use crate::{AuthError, CredentialSource, CredentialSourceConfig, CredentialSourceResolver};

/// Resolves credential source configuration to concrete sources.
///
/// Handles file path resolution with env var overrides and home directory expansion.
#[derive(Clone, Debug)]
pub struct FileCredentialResolver;

impl FileCredentialResolver {
    /// Create a new file credential resolver.
    pub fn new() -> Self {
        Self
    }

    /// Expand `~` in a path to the user's home directory.
    fn expand_home(path: &str) -> PathBuf {
        if path.starts_with("~/") || path == "~" {
            if let Some(home) = dirs::home_dir() {
                let rest = if path == "~" { "" } else { &path[2..] };
                return home.join(rest);
            }
        }
        PathBuf::from(path)
    }

    /// Check if a file path is readable (exists and accessible).
    fn is_readable(path: &PathBuf) -> bool {
        path.exists() && path.is_file()
    }
}

impl Default for FileCredentialResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl CredentialSourceResolver for FileCredentialResolver {
    fn resolve(&self, config: &CredentialSourceConfig) -> Result<CredentialSource, AuthError> {
        // Try file_path_env_override first (highest priority)
        if let Some(override_var) = &config.file_path_env_override {
            if let Ok(path_str) = std::env::var(override_var) {
                let path = Self::expand_home(&path_str);
                if Self::is_readable(&path) {
                    return Ok(CredentialSource::EnvVar(override_var.clone()));
                }
            }
        }

        // Try file_path next (medium priority)
        if let Some(file_path) = &config.file_path {
            let path = Self::expand_home(file_path);
            if Self::is_readable(&path) {
                // Return the expanded path as an env var source
                // In practice, the resolved path will be passed to OAuthTokenSourceFactory
                // For now, return it as a synthetic env var name that the resolver knows about
                return Ok(CredentialSource::EnvVar(
                    path.to_string_lossy().into_owned(),
                ));
            }
        }

        // Fall back to env_var (lowest priority)
        if let Some(env_var) = &config.env_var {
            return Ok(CredentialSource::EnvVar(env_var.clone()));
        }

        // No sources configured or available
        Err(AuthError::MissingCredential(
            "no credential source configured: \
             set env_var, file_path, or file_path_env_override in credential_source config"
                .into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_expand_home_with_tilde() {
        let expanded = FileCredentialResolver::expand_home("~/.credentials.json");
        assert!(expanded.is_absolute());
        assert!(expanded.to_string_lossy().contains(".credentials.json"));
    }

    #[test]
    fn test_expand_home_plain_path() {
        let expanded = FileCredentialResolver::expand_home("/etc/credentials.json");
        assert_eq!(expanded.as_os_str(), "/etc/credentials.json");
    }

    #[test]
    fn test_is_readable_existing_file() {
        let file = NamedTempFile::new().expect("create temp file");
        let path = file.path().to_path_buf();
        assert!(FileCredentialResolver::is_readable(&path));
    }

    #[test]
    fn test_is_readable_nonexistent_file() {
        let path = PathBuf::from("/nonexistent/path/to/file.json");
        assert!(!FileCredentialResolver::is_readable(&path));
    }

    #[test]
    fn test_resolve_with_env_var_only() {
        let resolver = FileCredentialResolver::new();
        let cfg = CredentialSourceConfig::new().with_env_var("API_KEY");
        let result = resolver.resolve(&cfg).expect("resolve");
        assert_eq!(result, CredentialSource::EnvVar("API_KEY".into()));
    }

    #[test]
    fn test_resolve_with_file_path() {
        let resolver = FileCredentialResolver::new();
        let file = NamedTempFile::new().expect("create temp file");
        let path_str = file.path().to_string_lossy().into_owned();

        let cfg = CredentialSourceConfig::new().with_file_path(&path_str);
        let result = resolver.resolve(&cfg).expect("resolve");

        // Should return the file path as a credential source
        if let CredentialSource::EnvVar(source_name) = result {
            assert_eq!(source_name, path_str);
        } else {
            panic!("expected EnvVar, got something else");
        }
    }

    #[test]
    fn test_resolve_file_path_priority_over_env_var() {
        let resolver = FileCredentialResolver::new();
        let file = NamedTempFile::new().expect("create temp file");
        let path_str = file.path().to_string_lossy().into_owned();

        let cfg = CredentialSourceConfig::new()
            .with_env_var("API_KEY")
            .with_file_path(&path_str);
        let result = resolver.resolve(&cfg).expect("resolve");

        // File path should be preferred over env var
        if let CredentialSource::EnvVar(source_name) = result {
            assert_eq!(source_name, path_str);
        } else {
            panic!("expected file path to be preferred");
        }
    }

    #[test]
    fn test_resolve_env_override_priority() {
        let resolver = FileCredentialResolver::new();
        let file = NamedTempFile::new().expect("create temp file");
        let path_str = file.path().to_string_lossy().into_owned();

        // Set up env var override
        std::env::set_var("TEST_CREDS_OVERRIDE", &path_str);

        let cfg = CredentialSourceConfig::new()
            .with_env_var("API_KEY")
            .with_file_path_env_override("TEST_CREDS_OVERRIDE");

        let result = resolver.resolve(&cfg).expect("resolve");

        // Override should be preferred
        if let CredentialSource::EnvVar(source_name) = result {
            assert_eq!(source_name, "TEST_CREDS_OVERRIDE");
        } else {
            panic!("expected override env var to be preferred");
        }

        std::env::remove_var("TEST_CREDS_OVERRIDE");
    }

    #[test]
    fn test_resolve_no_sources_returns_error() {
        let resolver = FileCredentialResolver::new();
        let cfg = CredentialSourceConfig::new();
        let result = resolver.resolve(&cfg);
        assert!(matches!(result, Err(AuthError::MissingCredential(_))));
    }

    #[test]
    fn test_resolve_missing_file_falls_back_to_env_var() {
        let resolver = FileCredentialResolver::new();
        let cfg = CredentialSourceConfig::new()
            .with_file_path("/nonexistent/file.json")
            .with_env_var("API_KEY");
        let result = resolver.resolve(&cfg).expect("resolve");
        assert_eq!(result, CredentialSource::EnvVar("API_KEY".into()));
    }
}
