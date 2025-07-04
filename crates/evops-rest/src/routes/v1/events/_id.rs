use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::{Path, State};

use evops_models::ApiResult;

use crate::AppState;
use crate::error::AddResponse as _;
use crate::types::{EventServiceFindRequest, EventServiceFindResponse};

mod images;

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::EventService.into())
}
pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route_with("/", get_with(self::get, self::get_docs), self::route_docs)
        .nest("/images", self::images::router())
}

fn get_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.EventService.Find")
        .description("Retrieves a single event by ID.")
        .response_bad_request()
        .response_not_found()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn get(
    State(state): State<AppState>,
    Path(request): Path<EventServiceFindRequest>,
) -> ApiResult<Json<EventServiceFindResponse>> {
    let request = request.id.into();
    let found_event = state.find_event(request).await?;

    let response_data = EventServiceFindResponse {
        event: found_event.into(),
    };
    Ok(Json(response_data))
}
