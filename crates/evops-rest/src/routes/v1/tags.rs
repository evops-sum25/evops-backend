use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::{Query, State};
use tap::TryConv as _;

use evops_models::{ApiError, ApiResult};

use crate::error::AddResponse as _;
use crate::types::{
    TagServiceCreateRequest, TagServiceCreateResponse, TagServiceListRequestQuery,
    TagServiceListResponse,
};
use crate::{AppState, Auth};

mod _tag_id;
mod suggestions;

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
        .nest("/{tag-id}", self::_tag_id::router())
        .nest("/suggestions", self::suggestions::router())
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
    Query(query): Query<TagServiceListRequestQuery>,
) -> ApiResult<Json<TagServiceListResponse>> {
    let last_id = query.last_id.map(Into::into);
    let limit = match query.limit {
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
        .response_unauthorized()
        .response_conflict()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn post(
    State(state): State<AppState>,
    Auth(user_id): Auth,
    Json(request): Json<TagServiceCreateRequest>,
) -> ApiResult<Json<TagServiceCreateResponse>> {
    let form = request.form.try_into()?;
    let tag_id = state.create_tag(form, user_id).await?;

    let response_data = TagServiceCreateResponse {
        tag_id: tag_id.into(),
    };
    Ok(Json(response_data))
}
