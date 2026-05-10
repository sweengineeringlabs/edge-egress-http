//! Interface counterpart for [`crate::core::metrics_http_outbound`].
//!
//! The concrete [`MetricsHttpOutbound`](crate::core::metrics_http_outbound::MetricsHttpOutbound)
//! decorator lives in `core/` and is wired by the SAF factory. Consumers program
//! to [`HttpOutbound`](super::port::http_outbound::HttpOutbound).
