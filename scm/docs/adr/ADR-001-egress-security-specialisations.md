# ADR-001: Egress Security Specialisations

**Status:** Accepted  
**Date:** 2026-06-12  
**Deciders:** phdsystems  
**Parent:** [ADR-015 — Three-Tier Security Layer Architecture](../../../../docs/3-architecture/adr/ADR-015-security-layer-architecture.md)  
**Affects:** `egress/auth`, `egress/tls`, `egress/oauth`

---

## Context

This ADR defines which security contracts are egress-specific and why they must not be promoted to `swe-edge-security`. It also captures the adaptations required when the shared layer is extended per ADR-015.

Egress security has one directional mandate:

> Attach the correct outbound credential to a request before it leaves the process boundary.

Everything in this workspace either serves that purpose or is out of scope.

---

## Egress-Specific Concepts (must stay here)

### Auth Strategies

The five auth strategies (None, Bearer, Basic, Header, AwsSigV4) are egress-only because:

- They mutate an outbound `reqwest::Request` by injecting `Authorization` or custom headers.
- Ingress does not attach credentials; it receives and verifies them.
- AWS Signature Version 4 is a request-signing protocol specific to outbound calls to AWS services.

`AuthConfig` (the TOML-backed strategy selector) and each strategy's implementation live here alongside `reqwest-middleware` integration.

### `OAuthTokenSource`

Egress calls downstream OAuth-protected APIs. The token refresh cycle (acquire → cache → refresh on expiry) is an outbound concern. There is no symmetric inbound concept — ingress verifies tokens presented by callers, it does not refresh tokens to call out.

`OAuthMiddleware` injects the acquired token into each outbound request. `OAuthConfig`, `OAuthCredentials`, and `OAuthProvider` are the TOML-backed configuration for the source implementation.

### Client TLS Identity (`ClientTlsConfig`)

Egress mTLS means presenting a client certificate to a downstream server. This uses `reqwest::Identity` (PKCS12 or PEM). The ingress equivalent is `tokio_rustls::TlsAcceptor` — the types are not interchangeable.

**After ADR-015 Step 3:** `TlsError` is replaced by re-exporting `swe_edge_security::TlsConfigError`. The config enum (`ClientTlsConfig`) stays here.

---

## Concepts Promoted to Shared (ADR-015)

### `CredentialResolver` and `CredentialSource`

These were originally defined in `egress/auth`. They are promoted because:

- The pattern "resolve a credential from an external source (env var, secret store) at runtime" is useful on both sides. Ingress could use `CredentialResolver` to supply JWT verification keys from env vars rather than hardcoding them in TOML.
- The trait signature `fn resolve(&self, source: &CredentialSource) -> Result<SecretString, SecurityError>` has no `reqwest` or egress dependency.

After promotion:
- `CredentialSource` and `CredentialResolver` are re-exported from shared in `egress/auth/api/credential/`
- `EnvCredentialResolver` (the process-env implementation) stays in `egress/auth/core/`
- No consumer-visible API change

---

## Relationship to the Shared Layer

After ADR-015 is implemented:

| Shared item used | How egress uses it |
|-----------------|---------------------|
| `CredentialResolver` trait | `EnvCredentialResolver` implements it |
| `CredentialSource` | Used by `AuthConfig` to describe where credentials come from |
| `Token` | Wraps bearer tokens before injection; prevents accidental debug logging |
| `SecurityError` | `AuthError` + `OAuthError` + `TlsConfigError` all convert `Into<SecurityError>` |
| `TlsConfigError` | `TlsError` is a type alias for this |

---

## What Egress Security Must Never Do

- Verify inbound bearer tokens (belongs in ingress)
- Extract `TenantId` from HTTP headers (belongs in ingress)
- Import from `ingress/*` crates
- Issue tokens or JWTs (not the role of a transport middleware)

---

## Error Conversion Requirement

All egress security errors must implement `Into<swe_edge_security::SecurityError>`:

| Error type | Variant mapped to | Status |
|-----------|------------------|--------|
| `AuthError` | `SecurityError::Auth(String)` | Done |
| `OAuthError` | `SecurityError::Token(String)` | **Missing — add in Step 3** |
| `TlsConfigError` | `SecurityError::Tls(String)` | Done (via shared type) |

This ensures a handler composing auth + oauth + tls can return a single `SecurityError` without manual bridging.

---

## Implementation Notes (ADR-015 Step 3)

1. `CredentialSource` + `CredentialResolver` in `api/credential/` → `pub use swe_edge_security::{CredentialSource, CredentialResolver}`
2. `egress/tls`: `TlsError` → `pub type TlsError = swe_edge_security::TlsConfigError`
3. `egress/oauth`: add `impl From<OAuthError> for swe_edge_security::SecurityError` (Token variant)
4. Bump workspace to **v0.5.x** (additive; CredentialResolver re-export is non-breaking; TlsError alias is non-breaking if TlsError is not part of the top-level pub surface)
