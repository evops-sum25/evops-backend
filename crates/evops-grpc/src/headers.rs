use http::HeaderName;

pub mod prelude {
    pub use super::{
        GRPC_MESSAGE, GRPC_STATUS, GRPC_STATUS_DETAILS_BIN, GRPC_TIMEOUT, X_GRPC_WEB, X_USER_AGENT,
    };
}

pub const GRPC_TIMEOUT: HeaderName = HeaderName::from_static("grpc-timeout");
pub const X_GRPC_WEB: HeaderName = HeaderName::from_static("x-grpc-web");
pub const X_USER_AGENT: HeaderName = HeaderName::from_static("x-user-agent");
pub const GRPC_STATUS: HeaderName = HeaderName::from_static("grpc-status");
pub const GRPC_MESSAGE: HeaderName = HeaderName::from_static("grpc-message");
pub const GRPC_STATUS_DETAILS_BIN: HeaderName = HeaderName::from_static("grpc-status-details-bin");
