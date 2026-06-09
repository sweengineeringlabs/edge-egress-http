//! SAF facade — public surface for the HTTP egress transport crate.

mod transport_svc;

pub use crate::api::error::HttpEgressBuildError;
pub use crate::api::error::HttpEgressError;
pub use crate::api::types::default::TransportConfig;
pub use crate::api::types::metrics::MetricsHttpEgress as MetricsEgress;
pub use crate::api::types::metrics::ObservationConfig;
pub use crate::api::types::validator::AlwaysValidConfig;
pub use crate::api::types::validator::HttpConfigValidator as HttpConfigValidatorAlias;
pub use crate::api::types::validator::HttpEgressObject as DefaultEgress;
pub use crate::api::types::validator::ValidatableHttpConfig;
pub use crate::api::types::validator::ValidatorObject as DefaultValidatorAlias;
pub use crate::api::types::HttpEgressResult;
pub use crate::api::types::{
    FormPart, HttpAuth, HttpBody, HttpConfig, HttpConfigBuilder, HttpMethod, HttpRequest,
    HttpRequestBuilder, HttpResponse, HttpStreamResponse, HttpTransportSvc, SseEvent, SseStream,
    WsChannel, WsMessage, WsReceiver, WsSender,
};
pub use crate::api::types::{HttpEgress, HttpStream};
