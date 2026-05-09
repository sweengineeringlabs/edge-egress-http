//! Interface counterpart for [`crate::core::default_http_outbound`].
//!
//! The concrete [`DefaultHttpOutbound`] struct lives in `core/` and is
//! assembled via SAF factory functions. This module documents the interface
//! contract — consumers program to [`HttpOutbound`](super::port::http_outbound::HttpOutbound).
