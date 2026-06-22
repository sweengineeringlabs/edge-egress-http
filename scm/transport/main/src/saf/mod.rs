//! SAF facade — public surface for the HTTP egress transport crate.

mod http;
mod metrics_http_egress_spec_svc;
mod transport_svc;
mod validator_svc;

pub(crate) use crate::api::error::HttpEgressBuildError;
pub(crate) use crate::api::error::HttpEgressError;
pub(crate) use crate::api::types::default::TransportConfig;
pub(crate) use crate::api::types::metrics::MetricsHttpEgress as MetricsEgress;
pub(crate) use crate::api::types::metrics::ObservationConfig;
pub(crate) use crate::api::types::validator::AlwaysValidConfig;
pub(crate) use crate::api::types::validator::HttpConfigValidator as HttpConfigValidatorAlias;
pub(crate) use crate::api::types::validator::HttpEgressObject as DefaultEgress;
pub(crate) use crate::api::types::validator::ValidatableHttpConfig;
pub(crate) use crate::api::types::validator::ValidatorObject as DefaultValidatorAlias;
pub(crate) use crate::api::types::HttpEgressResult;
pub(crate) use crate::api::types::{
    FormPart, HttpAuth, HttpBody, HttpConfig, HttpConfigBuilder, HttpMethod, HttpRequest,
    HttpRequestBuilder, HttpResponse, HttpStreamResponse, HttpTransportSvc, SseEvent, SseStream,
    WsChannel, WsMessage, WsReceiver, WsSender,
};
pub(crate) use crate::api::types::{HttpEgress, HttpStream};
pub(crate) use edge_domain::SecurityContext;
