//! Validated AWS SigV4 strategy configuration.

use secrecy::SecretString;

/// Validated configuration produced by [`AwsSigV4StrategyBuilder::build_config`](super::aws_sig_v4_strategy_builder::AwsSigV4StrategyBuilder::build_config).
pub struct AwsSigV4StrategyConfig {
    /// AWS access key ID.
    pub access_key_id: SecretString,
    /// AWS secret access key.
    pub secret_access_key: SecretString,
    /// Optional STS/IMDSv2 session token.
    pub session_token: Option<SecretString>,
    /// AWS region.
    pub region: String,
    /// AWS service identifier.
    pub service: String,
}

impl std::fmt::Debug for AwsSigV4StrategyConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AwsSigV4StrategyConfig")
            .field("access_key_id", &"<redacted>")
            .field("secret_access_key", &"<redacted>")
            .field(
                "session_token",
                &if self.session_token.is_some() {
                    "<set>"
                } else {
                    "<none>"
                },
            )
            .field("region", &self.region)
            .field("service", &self.service)
            .finish()
    }
}
