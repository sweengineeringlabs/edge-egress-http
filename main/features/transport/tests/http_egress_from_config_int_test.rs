//! End-to-end test for ADR-006 config-driven activation in `transport`.
//!
//! Proves the consumer experience: adding a `[tls]` section to `application.toml`
//! wires client TLS into the assembled egress; omitting it (or `enabled = false`)
//! leaves TLS off. We exploit the fact that `build_tls_layer` resolves the cert
//! eagerly — so a present `[tls]` with a missing cert path fails at build time,
//! which is direct evidence that the TLS layer was actually attached.

use swe_edge_configbuilder::ConfigLoaderFactory;
use swe_edge_egress_http_transport::{HttpEgressBuildError, HttpTransportSvc};
use tempfile::TempDir;

fn loader(content: &str) -> (TempDir, swe_edge_configbuilder::SectionLoaderImpl) {
    let dir = TempDir::new().expect("create temp dir");
    std::fs::write(dir.path().join("application.toml"), content).expect("write application.toml");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    (dir, loader)
}

/// @covers: http_egress_from_config — `[tls]` present wires the TLS layer; eager
/// cert resolution fails on the missing file, proving the layer was attached.
#[test]
fn test_tls_section_present_wires_tls_layer() {
    let (_d, l) = loader("[tls]\nkind = \"pem\"\npath = \"/definitely/missing-xyz.pem\"");
    let result = HttpTransportSvc::http_egress_from_config(&l);
    assert!(
        matches!(result, Err(HttpEgressBuildError::Tls(_))),
        "[tls] present must wire TLS and fail eagerly on the missing cert"
    );
}

/// @covers: http_egress_from_config — no `[tls]` section ⇒ TLS off; the egress
/// builds successfully without touching any cert.
#[test]
fn test_no_tls_section_builds_without_tls() {
    let (_d, l) = loader("[unrelated]\nkey = \"value\"");
    let result = HttpTransportSvc::http_egress_from_config(&l);
    assert!(
        result.is_ok(),
        "absent [tls] must build successfully with TLS off"
    );
}

/// @covers: http_egress_from_config — `enabled = false` disables a present
/// `[tls]` section (no cert resolution, builds clean).
#[test]
fn test_tls_enabled_false_disables_tls() {
    let (_d, l) = loader("[tls]\nkind = \"pem\"\npath = \"/missing.pem\"\nenabled = false");
    let result = HttpTransportSvc::http_egress_from_config(&l);
    assert!(
        result.is_ok(),
        "enabled = false must disable TLS — no cert resolution attempted"
    );
}

/// @covers: http_egress_from_config — an invalid `[tls]` (empty path) surfaces a
/// Config validation error, not a silent pass.
#[test]
fn test_tls_invalid_section_returns_config_error() {
    let (_d, l) = loader("[tls]\nkind = \"pem\"\npath = \"\"");
    let result = HttpTransportSvc::http_egress_from_config(&l);
    assert!(
        matches!(result, Err(HttpEgressBuildError::Config(_))),
        "invalid [tls] must surface a Config error from validate_enabled"
    );
}

// ── [retry] section ────────────────────────────────────────────────────────

const RETRY_TOML: &str = "[retry]\nmax_retries = 3\ninitial_interval_ms = 200\n\
    max_interval_ms = 10000\nmultiplier = 2.0\nretryable_statuses = [503]\n\
    retryable_methods = [\"GET\"]\n";

/// @covers: http_egress_from_config — a valid `[retry]` section is loaded and the
/// retry layer is wired (the egress builds successfully).
#[test]
fn test_retry_section_present_builds() {
    let (_d, l) = loader(RETRY_TOML);
    let result = HttpTransportSvc::http_egress_from_config(&l);
    assert!(
        result.is_ok(),
        "valid [retry] must build with the retry layer wired"
    );
}

/// @covers: http_egress_from_config — no `[retry]` section ⇒ retry omitted; builds.
#[test]
fn test_no_retry_section_builds() {
    let (_d, l) = loader("[unrelated]\nkey = \"value\"");
    let result = HttpTransportSvc::http_egress_from_config(&l);
    assert!(
        result.is_ok(),
        "absent [retry] must build with retry omitted"
    );
}

/// @covers: http_egress_from_config — an invalid `[retry]` (multiplier = 0)
/// surfaces a Config validation error.
#[test]
fn test_retry_invalid_section_returns_config_error() {
    let toml = "[retry]\nmax_retries = 3\ninitial_interval_ms = 200\n\
        max_interval_ms = 10000\nmultiplier = 0.0\nretryable_statuses = [503]\n\
        retryable_methods = [\"GET\"]\n";
    let (_d, l) = loader(toml);
    let result = HttpTransportSvc::http_egress_from_config(&l);
    assert!(
        matches!(result, Err(HttpEgressBuildError::Config(_))),
        "invalid [retry] (multiplier=0) must surface a Config error"
    );
}

/// @covers: http_egress_from_config — `[retry]` with `enabled = false` is omitted.
#[test]
fn test_retry_enabled_false_omits_retry() {
    let toml = "[retry]\nenabled = false\nmax_retries = 3\ninitial_interval_ms = 200\n\
        max_interval_ms = 10000\nmultiplier = 2.0\nretryable_statuses = [503]\n\
        retryable_methods = [\"GET\"]\n";
    let (_d, l) = loader(toml);
    let result = HttpTransportSvc::http_egress_from_config(&l);
    assert!(
        result.is_ok(),
        "enabled=false [retry] must build with retry omitted"
    );
}
