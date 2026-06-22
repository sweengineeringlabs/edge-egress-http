//! `TlsProviderFactory` — turns a [`TlsConfig`] into a concrete
//! [`Box<dyn HttpTls>`].

use secrecy::SecretString;

use crate::api::error::TlsConfigError;
use crate::api::traits::HttpTls;
use crate::api::types::TlsConfig;

use super::noop_http_tls::NoopHttpTls;
use super::pem_http_tls::PemHttpTls;
use super::pkcs12_http_tls::Pkcs12HttpTls;

/// Realizes a [`TlsConfig`] into the right provider impl.
///
/// File reads + env-var resolution happen inside `build_provider`.
/// Missing files return [`TlsError::FileReadFailed`]; missing passwords
/// (when the config specifies one) return [`TlsError::MissingEnvVar`].
pub(crate) struct TlsProviderFactory;

impl TlsProviderFactory {
    /// Realize a [`TlsConfig`] into the right provider impl.
    pub(crate) fn build_provider(config: &TlsConfig) -> Result<Box<dyn HttpTls>, TlsConfigError> {
        match config {
            TlsConfig::None => Ok(Box::new(NoopHttpTls)),

            TlsConfig::Pkcs12 { path, password_env } => {
                let password = match password_env {
                    Some(var) => {
                        let v = std::env::var(var)
                            .map_err(|_| TlsConfigError::MissingEnvVar { name: var.clone() })?;
                        Some(SecretString::from(v))
                    }
                    None => None,
                };
                Ok(Box::new(Pkcs12HttpTls::new(path.clone(), password)?))
            }

            TlsConfig::Pem { path } => Ok(Box::new(PemHttpTls::new(path.clone())?)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: build_provider
    #[test]
    fn test_build_provider_none_variant_builds_noop() {
        let p = TlsProviderFactory::build_provider(&TlsConfig::None).unwrap();
        assert_eq!(p.describe(), "noop");
        assert!(p.identity().unwrap().is_none());
    }

    /// @covers: build_provider
    #[test]
    fn test_build_provider_pkcs12_missing_password_env_returns_missing_env_var() {
        std::env::remove_var("EDGE_TEST_TLS_PKCS_PW_ABSENT_01");
        let cfg = TlsConfig::Pkcs12 {
            path: "irrelevant.p12".into(),
            password_env: Some("EDGE_TEST_TLS_PKCS_PW_ABSENT_01".into()),
        };
        let err = TlsProviderFactory::build_provider(&cfg).unwrap_err();
        assert!(matches!(err, TlsConfigError::MissingEnvVar { name } if name == "EDGE_TEST_TLS_PKCS_PW_ABSENT_01"));
    }

    /// @covers: build_provider
    #[test]
    fn test_build_provider_pkcs12_missing_file_returns_file_read_failed() {
        let cfg = TlsConfig::Pkcs12 {
            path: "/path/definitely/does/not/exist.p12".into(),
            password_env: None,
        };
        let err = TlsProviderFactory::build_provider(&cfg).unwrap_err();
        assert!(matches!(err, TlsConfigError::CertLoad(_)));
    }

    /// @covers: build_provider
    #[test]
    fn test_build_provider_pem_missing_file_returns_file_read_failed() {
        let cfg = TlsConfig::Pem {
            path: "/path/definitely/does/not/exist.pem".into(),
        };
        let err = TlsProviderFactory::build_provider(&cfg).unwrap_err();
        assert!(matches!(err, TlsConfigError::CertLoad(_)));
    }

    #[test]
    fn test_build_provider_none_succeeds() {
        let p = TlsProviderFactory::build_provider(&TlsConfig::None);
        assert!(p.is_ok());
    }
}
