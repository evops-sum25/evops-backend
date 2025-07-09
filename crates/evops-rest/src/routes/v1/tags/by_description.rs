use aide::axum::ApiRouter;
use aide::axum::routing::post_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::State;

use evops_models::ApiResult;

use crate::AppState;
use crate::error::AddResponse as _;
use crate::types::{TagServiceGetTagsByDescriptionRequest, TagServiceGetTagsByDescriptionResponse};

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::TagService.into())
}
pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().api_route_with(
        "/",
        post_with(self::post, self::post_docs),
        self::route_docs,
    )
}

fn post_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.TagService.GetTagsByDescription")
        .description("Get relevant tag IDs for an event description.")
        .response_bad_request()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn post(
    State(state): State<AppState>,
    Json(request): Json<TagServiceGetTagsByDescriptionRequest>,
) -> ApiResult<Json<TagServiceGetTagsByDescriptionResponse>> {
    let description: evops_models::EventDescription = request.description.try_into()?;
    let tags_ids = state.get_tags_by_description(description).await?;
    let response_data = TagServiceGetTagsByDescriptionResponse {
        tag_ids: tags_ids.into_iter().map(Into::into).collect(),
    };
    Ok(Json(response_data))
}
