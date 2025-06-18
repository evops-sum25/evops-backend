use aide::OperationIo;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

/// 500
#[derive(Error, Debug, OperationIo)]
#[aide(output_with = "String")]
#[error("{0}")]
pub struct InternalServerError(String);
impl IntoResponse for self::InternalServerError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0).into_response()
    }
}

impl From<String> for self::InternalServerError {
    fn from(value: String) -> Self {
        Self(value)
    }
}
