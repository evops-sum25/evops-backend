use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::{Query, State};

use evops_models::ApiResult;

use crate::error::AddResponse as _;
use crate::types::{TagServiceSuggestRequestQuery, TagServiceSuggestResponse};
use crate::{AppState, Auth};

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::TagService.into())
}
pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().api_route_with("/", get_with(self::get, self::get_docs), self::route_docs)
}

fn get_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.TagService.Suggest")
        .description("Get relevant tag IDs for an event description.")
        .response_bad_request()
        .response_unauthorized()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn get(
    _: Auth,
    State(state): State<AppState>,
    Query(query): Query<TagServiceSuggestRequestQuery>,
) -> ApiResult<Json<TagServiceSuggestResponse>> {
    let description: evops_models::EventDescription = query.description.try_into()?;
    let tags = state.suggest_tags(description).await?;
    let response_data = TagServiceSuggestResponse {
        tags: tags.into_iter().map(Into::into).collect(),
    };
    Ok(Json(response_data))
}
