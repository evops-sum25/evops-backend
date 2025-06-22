use thiserror::Error;

pub type ApiResult<T> = std::result::Result<T, crate::ApiError>;

#[derive(Debug, Error)]
#[error("{0}")]
#[cfg_attr(
    feature = "aide",
    derive(aide::OperationIo),
    aide(output_with = "String")
)]
pub enum ApiError {
    InvalidArgument(String),
    NotFound(String),
    AlreadyExists(String),
    #[error(transparent)]
    Db(#[from] diesel::result::Error),
    Other(String),
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        use axum::http::StatusCode;

        match self {
            ApiError::InvalidArgument(e) => (StatusCode::UNPROCESSABLE_ENTITY, e),
            ApiError::NotFound(e) => (StatusCode::NOT_FOUND, e),
            ApiError::AlreadyExists(e) => (StatusCode::CONFLICT, e),
            ApiError::Db(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ApiError::Other(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
        }
        .into_response()
    }
}

#[cfg(feature = "tonic")]
impl From<ApiError> for tonic::Status {
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::InvalidArgument(e) => Self::invalid_argument(e),
            ApiError::NotFound(e) => Self::not_found(e),
            ApiError::AlreadyExists(e) => Self::already_exists(e),
            ApiError::Db(e) => Self::internal(e.to_string()),
            ApiError::Other(e) => Self::unknown(e),
        }
    }
}
