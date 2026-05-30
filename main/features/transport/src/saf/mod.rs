//! SAF facade — public surface for the HTTP egress transport crate.

mod edge_egress_http_transport_svc;

pub use crate::api::default_http_egress::{DefaultHttpEgress as DefaultEgress, TransportConfig};
pub use crate::api::http::{HttpEgressBuildError, HttpEgressConfig, HttpEgressConfigBuilder};
pub use crate::api::metrics_http_egress::{MetricsHttpEgress as MetricsEgress, ObservationConfig};
pub use crate::api::port::{HttpEgress, HttpEgressError, HttpEgressResult, HttpStream};
pub use crate::api::types::{
    FormPart, HttpAuth, HttpBody, HttpConfig, HttpConfigBuilder, HttpMethod, HttpRequest,
    HttpRequestBuilder, HttpResponse, HttpStreamResponse, HttpTransportSvc, SseEvent, SseStream,
    WsChannel, WsMessage, WsReceiver, WsSender,
};
pub use crate::api::validator::{
    AlwaysValidConfig, DefaultValidator as DefaultValidatorAlias,
    HttpConfigValidator as HttpConfigValidatorAlias, ValidatableHttpConfig,
};
