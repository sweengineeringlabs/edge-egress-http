//! Rate API trait declarations.

pub mod processor;
pub use processor::Processor;

pub mod validator;
pub use validator::Validator;

pub(crate) mod rate_bucket_ops;
pub(crate) use rate_bucket_ops::RateBucketOps;
