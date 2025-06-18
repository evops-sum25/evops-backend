use aide::OperationIo;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

/// 409
#[derive(Error, Debug, OperationIo)]
#[aide(output_with = "String")]
#[error("{0}")]
pub struct ConflictError(String);
impl IntoResponse for self::ConflictError {
    fn into_response(self) -> Response {
        (StatusCode::CONFLICT, self.0).into_response()
    }
}

impl From<String> for ConflictError {
    fn from(value: String) -> Self {
        Self(value)
    }
}
