use aide::OperationIo;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

/// 422
#[derive(Error, Debug, OperationIo)]
#[aide(output_with = "String")]
#[error("{0}")]
pub struct UnprocessableEntityError(String);
impl IntoResponse for self::UnprocessableEntityError {
    fn into_response(self) -> Response {
        (StatusCode::UNPROCESSABLE_ENTITY, self.0).into_response()
    }
}

impl From<String> for self::UnprocessableEntityError {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<evops_models::EventDescriptionError> for self::UnprocessableEntityError {
    fn from(value: evops_models::EventDescriptionError) -> Self {
        value.to_string().into()
    }
}

impl From<evops_models::EventTitleError> for self::UnprocessableEntityError {
    fn from(value: evops_models::EventTitleError) -> Self {
        value.to_string().into()
    }
}

impl From<evops_models::TagNameError> for self::UnprocessableEntityError {
    fn from(value: evops_models::TagNameError) -> Self {
        value.to_string().into()
    }
}

impl From<evops_models::TagAliasError> for self::UnprocessableEntityError {
    fn from(value: evops_models::TagAliasError) -> Self {
        value.to_string().into()
    }
}

impl From<evops_models::UserNameError> for self::UnprocessableEntityError {
    fn from(value: evops_models::UserNameError) -> Self {
        value.to_string().into()
    }
}
