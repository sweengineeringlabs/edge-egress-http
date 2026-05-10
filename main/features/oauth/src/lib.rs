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
//! use swe_edge_egress_oauth::{builder, OAuthTokenSource, Result};
//!
//! #[derive(Debug)]
//! struct MyTokenSource;
//!
//! #[async_trait::async_trait]
//! impl OAuthTokenSource for MyTokenSource {
//!     async fn get_access_token(&self) -> Result<String> {
//!         Ok("my-token".to_string())
//!     }
//! }
//!
//! # fn main() -> std::result::Result<(), swe_edge_egress_oauth::Error> {
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
mod gateway;
mod saf;

pub use gateway::*;
