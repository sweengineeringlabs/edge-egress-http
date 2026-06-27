//! `Pkcs12HttpTls` — PKCS#12-bundle identity provider.

use secrecy::{ExposeSecret, SecretString};

use crate::api::error::TlsConfigError;
use crate::api::traits::HttpTls;

pub(crate) struct Pkcs12HttpTls {
    /// Pre-loaded DER bytes.
    der_bytes: Vec<u8>,
    /// Optional password (None for passwordless bundles).
    password: Option<SecretString>,
    /// Source path — kept for Debug / diagnostics.
    path: String,
}

impl std::fmt::Debug for Pkcs12HttpTls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pkcs12HttpTls")
            .field("path", &self.path)
            .field("bytes", &format!("<{} bytes>", self.der_bytes.len()))
            .field(
                "password",
                &if self.password.is_some() {
                    "<set>"
                } else {
                    "<none>"
                },
            )
            .finish()
    }
}

impl Pkcs12HttpTls {
    /// Construct by reading the .p12 / .pfx file into memory.
    /// Password (if any) is pre-resolved from env var before
    /// this call; see `TlsProviderFactory::build_provider`.
    pub(crate) fn new(
        path: String,
        password: Option<SecretString>,
    ) -> Result<Self, TlsConfigError> {
        let der_bytes = std::fs::read(&path).map_err(|e| {
            TlsConfigError::CertLoad(format!("could not read file {}: {}", path, e))
        })?;
        Ok(Self {
            der_bytes,
            password,
            path,
        })
    }
}

impl HttpTls for Pkcs12HttpTls {
    fn describe(&self) -> &'static str {
        const LABEL: &str = "pkcs12";
        LABEL
    }

    fn identity(&self) -> Result<Option<reqwest::Identity>, TlsConfigError> {
        let password = self
            .password
            .as_ref()
            .map(|p| p.expose_secret().to_string())
            .unwrap_or_default();
        let identity = reqwest::Identity::from_pkcs12_der(&self.der_bytes, &password)
            .map_err(|e| TlsConfigError::CertParse(format!("invalid pkcs12 data: {}", e)))?;
        Ok(Some(identity))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_reads_file_bytes_into_struct() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.p12");
        std::fs::write(&path, b"fake-pkcs12-bytes").unwrap();
        let p = Pkcs12HttpTls::new(path.to_str().unwrap().to_string(), None).unwrap();
        let dbg = format!("{p:?}");
        assert!(
            dbg.contains("17 bytes"),
            "debug must show byte count: {dbg}"
        );
        assert!(
            dbg.contains("<none>"),
            "debug must show password absent: {dbg}"
        );
    }

    /// @covers: new
    #[test]
    fn test_new_with_password_stores_password_flag() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test2.p12");
        std::fs::write(&path, b"bytes").unwrap();
        let p = Pkcs12HttpTls::new(
            path.to_str().unwrap().to_string(),
            Some(secrecy::SecretString::from("secret".to_string())),
        )
        .unwrap();
        let dbg = format!("{p:?}");
        assert!(
            dbg.contains("<set>"),
            "debug must show password is set: {dbg}"
        );
        assert!(
            !dbg.contains("secret"),
            "password must not leak into debug: {dbg}"
        );
    }

    /// @covers: new
    #[test]
    fn test_new_missing_file_returns_file_read_failed() {
        let err =
            Pkcs12HttpTls::new("/path/definitely/does/not/exist.p12".into(), None).unwrap_err();
        assert!(matches!(err, TlsConfigError::CertLoad(_)));
    }

    /// @covers: identity
    #[test]
    fn test_identity_on_invalid_der_bytes_returns_invalid_certificate() {
        let p = Pkcs12HttpTls {
            der_bytes: b"not-really-pkcs12".to_vec(),
            password: Some(SecretString::from("whatever".to_string())),
            path: "<stub>".into(),
        };
        let err = p.identity().unwrap_err();
        assert!(matches!(err, TlsConfigError::CertParse(_)));
    }

    /// @covers: describe
    #[test]
    fn test_describe_returns_pkcs12_label() {
        let p = Pkcs12HttpTls {
            der_bytes: vec![],
            password: None,
            path: "<stub>".into(),
        };
        assert_eq!(p.describe(), "pkcs12");
    }

    #[test]
    fn test_fmt_does_not_panic() {
        let p = Pkcs12HttpTls {
            der_bytes: vec![],
            password: None,
            path: "<stub>".into(),
        };
        let _ = format!("{:?}", p);
    }
}
