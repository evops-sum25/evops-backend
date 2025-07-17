use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::{Path, State};

use evops_models::ApiResult;

use crate::AppState;
use crate::error::AddResponse as _;
use crate::types::{
    TagServiceDeleteRequestPath, TagServiceFindRequestPath, TagServiceFindResponse,
};

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::TagService.into())
}
pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().api_route_with(
        "/",
        get_with(self::get, self::get_docs).delete_with(self::delete, self::delete_docs),
        self::route_docs,
    )
}

fn get_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.TagService.Find")
        .description("Retrieves a tag by ID.")
        .response_bad_request()
        .response_not_found()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn get(
    State(state): State<AppState>,
    Path(path): Path<TagServiceFindRequestPath>,
) -> ApiResult<Json<TagServiceFindResponse>> {
    let tag_id = path.tag_id.into();
    let found_tag = state.find_tag(tag_id).await?;

    let response_data = TagServiceFindResponse {
        tag: found_tag.into(),
    };
    Ok(Json(response_data))
}

fn delete_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.TagService.Delete")
        .description("Deletes a tag by ID.")
        .response_bad_request()
        .response_not_found()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn delete(
    State(state): State<AppState>,
    Path(path): Path<TagServiceDeleteRequestPath>,
) -> ApiResult<()> {
    let id = path.tag_id.into();
    state.delete_tag(id).await
}
