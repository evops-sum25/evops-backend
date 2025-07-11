use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::{Path, State};

use evops_models::ApiResult;

use crate::AppState;
use crate::error::AddResponse as _;
use crate::types::{
    TagServiceDeleteRequest, TagServiceFindRequest, TagServiceFindResponse, TagServiceUpdateRequest,
};

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::TagService.into())
}
pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().api_route_with(
        "/",
        {
            get_with(self::get, self::get_docs)
                .put_with(self::put, self::put_docs)
                .delete_with(self::delete, self::delete_docs)
        },
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
    Path(path): Path<TagServiceFindRequest>,
) -> ApiResult<Json<TagServiceFindResponse>> {
    let request = path.id.into();
    let found_tag = state.find_tag(request).await?;

    let response_data = TagServiceFindResponse {
        tag: found_tag.into(),
    };
    Ok(Json(response_data))
}

fn delete_docs(mut o: TransformOperation) -> TransformOperation {
    o.inner_mut().deprecated = true;
    o.summary("evops.api.v1.TagService.Delete")
        .description("Deletes a tag by ID.")
        .response_bad_request()
        .response_not_found()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn delete(Path(_request): Path<TagServiceDeleteRequest>) -> ApiResult<()> {
    todo!();
}

fn put_docs(mut o: TransformOperation) -> TransformOperation {
    o.inner_mut().deprecated = true;
    o.summary("evops.api.v1.TagService.Update")
        .description("Updates a tag by ID.")
        .response_bad_request()
        .response_not_found()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn put(Json(_request): Json<TagServiceUpdateRequest>) -> ApiResult<()> {
    todo!();
}
