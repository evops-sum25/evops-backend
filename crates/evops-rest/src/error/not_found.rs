use aide::OperationIo;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

/// 404
#[derive(Error, Debug, OperationIo)]
#[aide(output_with = "String")]
#[error("{0}")]
pub struct NotFoundError(String);
impl IntoResponse for self::NotFoundError {
    fn into_response(self) -> Response {
        (StatusCode::NOT_FOUND, self.0).into_response()
    }
}

impl From<String> for self::NotFoundError {
    fn from(value: String) -> Self {
        Self(value)
    }
}
