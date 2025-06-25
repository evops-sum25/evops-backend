use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::{Query, State};

use evops_models::ApiResult;

use crate::error::AddResponse as _;

mod _id;

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::EventService.into())
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
    o.summary("evops.api.v1.EventService.List")
        .description("Lists all events.")
        .response_bad_request()
        .response_internal_server_error()
}

async fn get(
    State(state): State<crate::AppState>,
    Query(request): Query<crate::types::EventServiceListRequest>,
) -> ApiResult<Json<crate::types::EventServiceListResponse>> {
    Ok(Json({
        state
            .list_events(request.try_into()?)
            .await?
            .into()
    }))
}

fn post_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.EventService.Create")
        .description("Creates a new event.")
        .response_bad_request()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn post(
    State(state): State<crate::AppState>,
    Json(request): Json<crate::types::EventServiceCreateRequest>,
) -> ApiResult<Json<crate::types::EventServiceCreateResponse>> {
    Ok(Json(state.create_event(request.try_into()?).await?.into()))
}
