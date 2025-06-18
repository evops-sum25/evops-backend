use aide::OperationIo;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

/// 400
#[derive(Error, Debug, OperationIo)]
#[aide(output_with = "String")]
#[error("{0}")]
pub struct BadRequestError(String);
impl IntoResponse for self::BadRequestError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.0).into_response()
    }
}

impl From<String> for self::BadRequestError {
    fn from(value: String) -> Self {
        Self(value)
    }
}
