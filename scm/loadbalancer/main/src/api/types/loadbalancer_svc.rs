//! `LoadbalancerSvc` — zero-size service struct with factory methods.

/// Zero-size marker with factory methods for the loadbalancer middleware.
///
/// All construction goes through `LoadbalancerSvc::build_layer` or the SAF
/// free functions; consumers never name core types directly.
pub struct LoadbalancerSvc;
