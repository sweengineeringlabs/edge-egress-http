//! `swe_edge_egress_oauth` — OAuth2 token-refresh middleware for edge egress.
//!
//! Defines the [`OAuthTokenSource`] trait and a `reqwest-middleware` shell
//! ([`OAuthMiddleware`]) that wraps it. Concrete implementations (credential
//! file loaders, token-refresh HTTP calls) live in consumer crates.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use std::sync::Arc;
//! use futures::future::BoxFuture;
//! use swe_edge_egress_oauth::{builder, OAuthBuilderOps, OAuthTokenSource, Result};
//!
//! #[derive(Debug)]
//! struct MyTokenSource;
//!
//! impl OAuthTokenSource for MyTokenSource {
//!     fn get_access_token(&self) -> BoxFuture<'_, Result<String>> {
//!         Box::pin(async { Ok("my-token".to_string()) })
//!     }
//! }
//!
//! # fn main() -> std::result::Result<(), swe_edge_egress_oauth::OAuthError> {
//! let mw = builder()
//!     .with_token_source(Arc::new(MyTokenSource))
//!     .build()?;
//!
//! let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
//!     .with(mw)
//!     .build();
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]

mod api;
mod core;
mod saf;

mod gateway;
pub use gateway::*;
