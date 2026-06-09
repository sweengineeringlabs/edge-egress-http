//! swe-edge-egress-loadbalancer — Client-side load-balancer middleware.
//!
//! Provides a [`LoadbalancerLayer`] that plugs into `reqwest_middleware::ClientBuilder`
//! and rewrites the request URL to a healthy backend selected by [`LoadbalancerConfig`]
//! strategy (round-robin, weighted, or least-connections).

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use saf::*;
