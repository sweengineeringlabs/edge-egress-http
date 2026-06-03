//! SAF facade — public surface for the HTTP egress transport crate.

mod transport_svc;

pub use crate::api::default::http::egress::{HttpEgressObject as DefaultEgress, TransportConfig};
pub use crate::api::http::HttpEgressBuildError;
pub use crate::api::metrics::{MetricsHttpEgress as MetricsEgress, ObservationConfig};
pub use crate::api::port::{HttpEgress, HttpEgressError, HttpEgressResult, HttpStream};
pub use crate::api::types::{
    FormPart, HttpAuth, HttpBody, HttpConfig, HttpConfigBuilder, HttpMethod, HttpRequest,
    HttpRequestBuilder, HttpResponse, HttpStreamResponse, HttpTransportSvc, SseEvent, SseStream,
    WsChannel, WsMessage, WsReceiver, WsSender,
};
pub use crate::api::validator::{
    AlwaysValidConfig, HttpConfigValidator as HttpConfigValidatorAlias, ValidatableHttpConfig,
    ValidatorObject as DefaultValidatorAlias,
};
