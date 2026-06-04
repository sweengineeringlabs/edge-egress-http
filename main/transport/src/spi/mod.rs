//! Extension hooks for downstream consumers.
//!
//! The presence of `spi/` signals that `saf/` intentionally returns
//! `Box<dyn Trait>` — consumers may substitute their own implementations.

pub(crate) mod egress;
