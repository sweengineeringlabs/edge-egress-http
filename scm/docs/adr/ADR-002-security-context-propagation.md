# ADR-002: Security Context Propagation — egress/http credential resolver + outbound context

**Status:** Accepted  
**Date:** 2026-06-12  
**Governing ADR:** [ADR-017](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-017-security-context-propagation.md) — Security Context Propagation Pipeline  
**See also:** [ADR-001](ADR-001-egress-security-specialisations.md) — egress HTTP security specialisations

---

## Mandate

Update `CredentialResolver` implementations to accept `SecurityContext`. Add `send_with_context` to `HttpOutbound` as the new primary method; `send` becomes a shim.

---

## `CredentialResolver` change (breaking)

```rust
// Before
fn resolve(&self, source: &CredentialSource) -> Result<SecretString, SecurityError>;

// After
fn resolve(&self, source: &CredentialSource, ctx: &SecurityContext) -> Result<SecretString, SecurityError>;
```

**Affected implementations:** `DefaultHttpAuth`, `EnvCredentialResolver`.

Until ADR-018 (per-tenant credential selection) lands, add `let _ = ctx;` — no behaviour change, but the signature is future-ready.

---

## `HttpOutbound` change (additive)

```rust
pub trait HttpOutbound: Send + Sync {
    async fn send(&self, req: HttpRequest) -> Result<HttpResponse, EgressError> {
        self.send_with_context(req, &SecurityContext::unauthenticated()).await
    }
    async fn send_with_context(
        &self,
        req: HttpRequest,
        ctx: &SecurityContext,
    ) -> Result<HttpResponse, EgressError>;
}
```

All existing `HttpOutbound` implementations override only `send_with_context`. The `send()` shim is inherited for free — no call-site migration required.

---

## Dependency change

```toml
edge-domain = { ..., features = ["security"] }
```

---

## Cascade position

Steps 6 and 11 of 11. `CredentialResolver` change (step 6) blocked on: swe-edge-security ADR-001. `HttpOutbound` with_context (step 11) can proceed in parallel. Both unblock: bootstrap final assembly.
