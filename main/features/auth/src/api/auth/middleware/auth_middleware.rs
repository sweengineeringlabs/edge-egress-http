//! Interface counterpart for `core::auth::middleware::auth_middleware`.

/// Interface counterpart for the auth middleware layer — re-exports the
/// concrete [`AuthMiddleware`](crate::api::types::auth::AuthMiddleware) type
/// for api/ structural compliance (SEA rule 161).
#[expect(
    dead_code,
    reason = "SEA api/ type alias anchor — intentionally unused"
)]
pub type AuthMiddleware = crate::api::types::auth::AuthMiddleware;
