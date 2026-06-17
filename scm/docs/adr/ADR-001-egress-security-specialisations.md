# ADR-001: Egress HTTP Security — Local Contract

**Status:** Accepted  
**Date:** 2026-06-12  
**Governing ADR:** [ADR-015](../../../../../docs/3-architecture/adr/ADR-015-security-layer-architecture.md) (rules R1–R7, shared surface, cascade)

---

## Mandate

Attach the correct outbound credential to every HTTP request before it leaves the process. Never verify inbound tokens or perform tenant resolution.

---

## What Lives Here

| Item | Crate | Why not shared |
|------|-------|---------------|
| `AuthConfig` | auth | Outbound strategy selector (None/Bearer/Basic/Header/AwsSigV4) |
| `AuthStrategy` impls (×5) | auth | Mutate `reqwest::Request`; no ingress analogue |
| `EnvCredentialResolver` | auth | Implements shared `CredentialResolver` via process env |
| `AuthError` | auth | Auth strategy + credential resolution failures |
| `OAuthTokenSource` | oauth | Token refresh for outbound calls; no ingress analogue |
| `OAuthMiddleware` | oauth | Injects token into `reqwest` requests |
| `OAuthError` | oauth | Token refresh failures |
| `ClientTlsConfig` | tls | Client mTLS identity (Pkcs12/Pem/None) for `reqwest::ClientBuilder` |

## What Is Re-exported from Shared

| Re-export | Source |
|-----------|--------|
| `CredentialResolver` | `swe_edge_security::CredentialResolver` |
| `CredentialSource` | `swe_edge_security::CredentialSource` |
| `TlsError` (alias) | `swe_edge_security::TlsConfigError` |
