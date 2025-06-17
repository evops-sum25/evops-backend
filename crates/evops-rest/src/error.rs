use aide::OperationIo;
use aide::transform::TransformOperation;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

pub use self::bad_request::BadRequestError;
pub use self::conflict::ConflictError;
pub use self::internal_server_error::InternalServerError;
pub use self::unprocessable_entity::UnprocessableEntityError;

mod bad_request;
mod conflict;
mod conversions;
mod internal_server_error;
mod unprocessable_entity;

#[derive(Error, Debug, OperationIo)]
#[error(transparent)]
#[aide(output_with = "String")]
pub enum Error {
    /// 400
    BadRequest(#[from] self::BadRequestError),
    /// 409
    Conflict(#[from] self::ConflictError),
    /// 422
    UnprocessableEntity(#[from] self::UnprocessableEntityError),
    /// 500
    #[allow(clippy::enum_variant_names)]
    InternalServerError(#[from] self::InternalServerError),
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

impl IntoResponse for self::Error {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest(e) => e.into_response(),
            Self::Conflict(e) => e.into_response(),
            Self::UnprocessableEntity(e) => e.into_response(),
            Self::InternalServerError(e) => e.into_response(),
        }
    }
}

impl AddResponse for TransformOperation<'_> {
    fn response_bad_request(self) -> Self {
        self.response_with::<400, self::BadRequestError, _>(|r| {
            r.description("Bad Request (e.g. invalid JSON syntax)")
        })
    }

    fn response_conflict(self) -> Self {
        self.response_with::<409, self::ConflictError, _>(|r| {
            r.description("Conflict (e.g. the entity exists in the database)")
        })
    }

    fn response_unprocessable_entity(self) -> Self {
        self.response_with::<422, self::UnprocessableEntityError, _>(|r| {
            r.description("Unprocessable Entity (e.g. wrong data types)")
        })
    }

    fn response_internal_server_error(self) -> Self {
        self.response_with::<500, self::InternalServerError, _>(|r| {
            r.description("Internal Server Error (sorry)")
        })
    }
}
