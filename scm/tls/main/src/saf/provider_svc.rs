//! SAF anchor for the `Provider` interface — SEA Rule 218 compliance.

use crate::api::traits::Provider;

/// Returns the provider's own label for use in log / trace output.
pub fn describe_tls_provider(provider: &impl Provider) -> &'static str {
    provider.describe()
}
