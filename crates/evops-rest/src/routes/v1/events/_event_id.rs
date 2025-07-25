use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::{Path, State};

use evops_models::ApiResult;

use crate::error::AddResponse as _;
use crate::types::{
    EventServiceDeleteRequestPath, EventServiceFindRequestPath, EventServiceFindResponse,
    EventServiceUpdateRequest, EventServiceUpdateRequestPath,
};
use crate::{AppState, Auth};

mod images;

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::EventService.into())
}
pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route_with(
            "/",
            {
                get_with(self::get, self::get_docs)
                    .delete_with(self::delete, self::delete_docs)
                    .put_with(self::put, self::put_docs)
            },
            self::route_docs,
        )
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
    Path(path): Path<EventServiceFindRequestPath>,
) -> ApiResult<Json<EventServiceFindResponse>> {
    let event_id = path.event_id.into();
    let found_event = state.find_event(event_id).await?;

    let response_data = EventServiceFindResponse {
        event: found_event.into(),
    };
    Ok(Json(response_data))
}

fn delete_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.EventService.Delete")
        .description("Deletes an event by ID.")
        .response_bad_request()
        .response_unauthorized()
        .response_forbidden()
        .response_not_found()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn delete(
    State(state): State<AppState>,
    Auth(user_id): Auth,
    Path(path): Path<EventServiceDeleteRequestPath>,
) -> ApiResult<()> {
    let request: evops_models::EventId = path.event_id.into();
    state.delete_event(request, user_id).await
}

fn put_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.EventService.Update")
        .description("Updates an event by ID.")
        .response_bad_request()
        .response_unauthorized()
        .response_forbidden()
        .response_not_found()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn put(
    State(state): State<AppState>,
    Auth(user_id): Auth,
    Path(path): Path<EventServiceUpdateRequestPath>,
    Json(request): Json<EventServiceUpdateRequest>,
) -> ApiResult<()> {
    let form = request.form.try_into()?;
    let event_id = path.event_id.into();
    state.update_event(event_id, user_id, form).await
}
