use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::State;

use crate::error::AddResponse as _;

mod _id;

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::TagService.into())
}
pub fn router() -> ApiRouter<crate::AppState> {
    ApiRouter::new()
        .api_route_with(
            "/",
            get_with(self::get, self::get_docs).post_with(self::post, self::post_docs),
            self::route_docs,
        )
        .nest("/{id}", self::_id::router())
}

fn get_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.TagService.List")
        .description("Lists all tags.")
        .response_bad_request()
        .response_unprocessable_entity()
        .response_internal_server_error()
}

async fn get(
    State(state): State<crate::AppState>,
    // Query(request): Query<crate::types::TagServiceListRequest>,
) -> crate::Result<Json<crate::types::TagServiceListResponse>> {
    Ok(Json({
        state
            .list_tags(crate::types::TagServiceListRequest.into())
            .await?
            .into()
    }))
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
    State(state): State<crate::AppState>,
    Json(request): Json<crate::types::TagServiceCreateRequest>,
) -> crate::Result<Json<crate::types::TagServiceCreateResponse>> {
    Ok(Json(state.create_tag(request.try_into()?).await?.into()))
}
