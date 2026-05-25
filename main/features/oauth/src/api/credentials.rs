//! OAuth credential types and provider config.

pub use crate::api::types::credentials::{OAuthConfig, OAuthCredentials, OAuthProvider};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_config_parses_claude() {
        let cfg = OAuthConfig::from_config(r#"provider = "claude""#).unwrap();
        assert_eq!(cfg.provider, OAuthProvider::Claude);
        assert!(cfg.credentials_path.is_none());
    }

    #[test]
    fn test_from_config_parses_google_with_path() {
        let cfg = OAuthConfig::from_config(
            r#"provider = "google"
credentials_path = "/custom/creds.json""#,
        )
        .unwrap();
        assert_eq!(cfg.provider, OAuthProvider::Google);
        assert_eq!(cfg.credentials_path.as_deref(), Some("/custom/creds.json"));
    }

    #[test]
    fn test_from_config_parses_open_ai() {
        let cfg = OAuthConfig::from_config(r#"provider = "open_ai""#).unwrap();
        assert_eq!(cfg.provider, OAuthProvider::OpenAi);
    }
}
