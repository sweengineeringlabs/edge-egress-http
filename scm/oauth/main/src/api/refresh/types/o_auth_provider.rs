//! `OAuthProvider` — identifies which OAuth provider's credential file to use.

use serde::Deserialize;

/// Identifies which provider's credential file and token endpoint to use.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OAuthProvider {
    /// Anthropic Claude — `~/.claude/.credentials.json`
    Claude,
    /// Google (Gemini CLI or gcloud ADC) — `~/.gemini/oauth_creds.json`
    /// or `~/.config/gcloud/application_default_credentials.json`
    Google,
    /// OpenAI — `~/.config/openai/credentials.json`
    OpenAi,
}
