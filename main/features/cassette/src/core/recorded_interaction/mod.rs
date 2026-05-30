//! Serializable shapes for one recorded request/response pair.
pub(crate) mod interaction;
pub(crate) mod request;
pub(crate) mod response;

pub(crate) use interaction::RecordedInteraction;
pub(crate) use request::RecordedRequest;
pub(crate) use response::RecordedResponse;
