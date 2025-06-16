use aide::OperationIo;
use aide::transform::TransformOperation;
use http::StatusCode;
use schemars::JsonSchema;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize, JsonSchema)]
#[error("{0}")]
pub struct Error(pub String);

#[derive(Error, Debug, OperationIo, Serialize, JsonSchema)]
#[aide(output_with = "String")]
#[error(transparent)]
pub struct BadRequest(#[from] pub Error);

#[derive(Error, Debug, OperationIo, Serialize, JsonSchema)]
#[aide(output_with = "String")]
#[error(transparent)]
pub struct Conflict(#[from] pub Error);

#[derive(Error, Debug, OperationIo, Serialize, JsonSchema)]
#[aide(output_with = "String")]
#[error(transparent)]
pub struct UnprocessableEntity(#[from] pub Error);

#[derive(Error, Debug, OperationIo, Serialize, JsonSchema)]
#[aide(output_with = "String")]
#[error(transparent)]
pub struct InternalServerError(#[from] pub Error);
impl From<eyre::Error> for InternalServerError {
    fn from(value: eyre::Error) -> Self {
        Self(Error(value.to_string()))
    }
}

pub trait AddResponse {
    /// 400
    fn response_bad_request(self) -> Self;

    /// 409
    fn response_conflict(self) -> Self;

    /// 422
    fn response_unprocessable_entity(self) -> Self;

    /// 500
    fn response_internal_server_error(self) -> Self;
}

impl AddResponse for TransformOperation<'_> {
    fn response_bad_request(self) -> Self {
        self.response_with::<400, self::BadRequest, _>(|r| {
            r.description(StatusCode::BAD_REQUEST.canonical_reason().unwrap())
        })
    }

    fn response_conflict(self) -> Self {
        self.response_with::<409, self::Conflict, _>(|r| {
            r.description(StatusCode::CONFLICT.canonical_reason().unwrap())
        })
    }

    fn response_unprocessable_entity(self) -> Self {
        self.response_with::<422, self::UnprocessableEntity, _>(|r| {
            r.description(StatusCode::UNPROCESSABLE_ENTITY.canonical_reason().unwrap())
        })
    }

    fn response_internal_server_error(self) -> Self {
        self.response_with::<500, self::InternalServerError, _>(|r| {
            r.description({
                StatusCode::INTERNAL_SERVER_ERROR
                    .canonical_reason()
                    .unwrap()
            })
        })
    }
}
