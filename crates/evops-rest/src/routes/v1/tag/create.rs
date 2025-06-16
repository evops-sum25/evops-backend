use aide::OperationIo;
use aide::axum::ApiRouter;
use aide::axum::routing::post_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::State;
use eyre::eyre;
use schemars::JsonSchema;
use serde::Serialize;
use thiserror::Error;

use crate::error::AddResponse as _;

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::TagService.into())
}
pub fn router() -> ApiRouter<crate::AppState> {
    ApiRouter::new().api_route_with(
        "/",
        post_with(self::post, self::post_docs),
        self::route_docs,
    )
}

fn post_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.TagService/Create")
        .response_bad_request()
        .response_conflict()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn post(
    State(state): State<crate::AppState>,
    Json(request): Json<crate::types::TagServiceCreateRequest>,
) -> Result<Json<crate::types::TagServiceCreateResponse>, PostError> {
    Ok(Json({
        state
            .create_tag(evops_types::CreateTagRequest::try_from(request)?)
            .await?
            .into()
    }))
}
#[derive(Error, Debug, OperationIo, Serialize, JsonSchema)]
#[aide(output_with = "String")]
#[error(transparent)]
enum PostError {
    BadRequest(#[from] crate::error::BadRequest),
    Conflict(#[from] crate::error::Conflict),
    UnprocessableEntity(#[from] crate::error::UnprocessableEntity),
    InternalServerError(#[from] crate::error::InternalServerError),
}

impl From<evops_core::errors::CreateTagError> for self::PostError {
    fn from(value: evops_core::errors::CreateTagError) -> Self {
        use evops_core::errors::CreateTagError as E;

        match value {
            E::Db(e) => {
                use evops_core::errors::db::CreateTagError as E;
                match e {
                    E::Duplicate(e) => {
                        Self::Conflict(crate::error::Conflict(crate::error::Error(e)))
                    }
                    E::Other(e) => Self::InternalServerError(eyre!("{e}").into()),
                }
            }
        }
    }
}
