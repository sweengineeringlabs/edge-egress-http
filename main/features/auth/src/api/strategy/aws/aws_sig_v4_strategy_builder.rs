//! Fluent builder for constructing `AwsSigV4Strategy` instances.
//!
//! Provides a builder pattern for the 5-field `AwsSigV4Strategy` struct
//! per Rule 91 (structs with 5+ fields require a builder).

use secrecy::SecretString;

use crate::api::error::AuthError;
pub use crate::api::strategy::aws::aws_sig_v4_strategy_config::AwsSigV4StrategyConfig;

/// Fluent builder for `AwsSigV4Strategy`.
///
/// Required fields: `access_key_id`, `secret_access_key`, `region`, `service`.
/// Optional: `session_token` (for STS/IMDSv2-issued temporary credentials).
#[derive(Default)]
pub struct AwsSigV4StrategyBuilder {
    access_key_id: Option<SecretString>,
    secret_access_key: Option<SecretString>,
    session_token: Option<SecretString>,
    region: Option<String>,
    service: Option<String>,
}

impl AwsSigV4StrategyBuilder {
    /// Create a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the AWS access key ID.
    pub fn with_access_key_id(mut self, v: impl Into<String>) -> Self {
        self.access_key_id = Some(SecretString::from(v.into()));
        self
    }

    /// Set the AWS secret access key.
    pub fn with_secret_access_key(mut self, v: impl Into<String>) -> Self {
        self.secret_access_key = Some(SecretString::from(v.into()));
        self
    }

    /// Set the optional STS/IMDSv2 session token.
    pub fn with_session_token(mut self, v: impl Into<String>) -> Self {
        self.session_token = Some(SecretString::from(v.into()));
        self
    }

    /// Set the AWS region (e.g. `"us-east-1"`).
    pub fn with_region(mut self, v: impl Into<String>) -> Self {
        self.region = Some(v.into());
        self
    }

    /// Set the AWS service identifier (e.g. `"s3"`, `"sts"`).
    pub fn with_service(mut self, v: impl Into<String>) -> Self {
        self.service = Some(v.into());
        self
    }

    /// Consume the builder and validate fields.
    ///
    /// Returns [`AuthError::InvalidHeaderValue`] if any required field is missing.
    pub fn build_config(self) -> Result<AwsSigV4StrategyConfig, AuthError> {
        let access_key_id = self.access_key_id.ok_or_else(|| {
            AuthError::InvalidHeaderValue(
                "AwsSigV4StrategyBuilder: access_key_id is required".into(),
            )
        })?;
        let secret_access_key = self.secret_access_key.ok_or_else(|| {
            AuthError::InvalidHeaderValue(
                "AwsSigV4StrategyBuilder: secret_access_key is required".into(),
            )
        })?;
        let region = self.region.ok_or_else(|| {
            AuthError::InvalidHeaderValue("AwsSigV4StrategyBuilder: region is required".into())
        })?;
        let service = self.service.ok_or_else(|| {
            AuthError::InvalidHeaderValue("AwsSigV4StrategyBuilder: service is required".into())
        })?;
        Ok(AwsSigV4StrategyConfig {
            access_key_id,
            secret_access_key,
            session_token: self.session_token,
            region,
            service,
        })
    }
}
