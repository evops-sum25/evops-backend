use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::{Query, State};
use tap::TryConv as _;

use evops_models::{ApiError, ApiResult};

use crate::AppState;
use crate::error::AddResponse as _;
use crate::types::{
    TagServiceCreateRequest, TagServiceCreateResponse, TagServiceListRequest,
    TagServiceListResponse,
};

mod _id;
mod by_description;

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::TagService.into())
}
pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route_with(
            "/",
            get_with(self::get, self::get_docs).post_with(self::post, self::post_docs),
            self::route_docs,
        )
        .nest("/{id}", self::_id::router())
        .nest("/by-description", self::by_description::router())
}

fn get_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.TagService.List")
        .description("Lists all tags.")
        .response_bad_request()
        .response_unprocessable_entity()
        .response_internal_server_error()
}

async fn get(
    State(state): State<AppState>,
    Query(request): Query<TagServiceListRequest>,
) -> ApiResult<Json<TagServiceListResponse>> {
    let last_id = request.last_id.map(Into::into);
    let limit = match request.limit {
        Some(lim) => Some({
            lim.try_conv::<evops_models::PgLimit>()
                .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
        }),
        None => None,
    };
    let tags = state.list_tags(last_id, limit).await?;

    let response_data = TagServiceListResponse {
        tags: tags.into_iter().map(Into::into).collect(),
    };
    Ok(Json(response_data))
}

fn post_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.TagService.Create")
        .description("Creates a new tag.")
        .response_bad_request()
        .response_conflict()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn post(
    State(state): State<AppState>,
    Json(request): Json<TagServiceCreateRequest>,
) -> ApiResult<Json<TagServiceCreateResponse>> {
    let form = request.form.try_into()?;
    let tag_id = state.create_tag(form).await?;

    let response_data = TagServiceCreateResponse {
        tag_id: tag_id.into(),
    };
    Ok(Json(response_data))
}
