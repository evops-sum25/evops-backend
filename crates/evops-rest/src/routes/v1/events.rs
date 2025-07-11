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
    EventServiceCreateRequest, EventServiceCreateResponse, EventServiceListRequest,
    EventServiceListResponse,
};

mod _id;
mod images;

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::EventService.into())
}
pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route_with(
            "/",
            get_with(self::get, self::get_docs).post_with(self::post, self::post_docs),
            self::route_docs,
        )
        .nest("/{id}", self::_id::router())
        .nest("/images", self::images::router())
}

fn get_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.EventService.List")
        .description("Lists all events.")
        .response_bad_request()
        .response_internal_server_error()
}
async fn get(
    State(state): State<AppState>,
    Query(request): Query<EventServiceListRequest>,
) -> ApiResult<Json<EventServiceListResponse>> {
    let last_id = request.last_id.map(Into::into);
    let limit = match request.limit {
        Some(lim) => Some({
            lim.try_conv::<evops_models::PgLimit>()
                .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
        }),
        None => None,
    };
    let events = state.list_events(last_id, limit).await?;

    let response_data = EventServiceListResponse {
        events: events.into_iter().map(Into::into).collect(),
    };
    Ok(Json(response_data))
}

fn post_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.EventService.Create")
        .description("Creates a new event.")
        .response_bad_request()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn post(
    State(state): State<AppState>,
    Json(request): Json<EventServiceCreateRequest>,
) -> ApiResult<Json<EventServiceCreateResponse>> {
    let form = request.form.try_into()?;
    let event_id = state.create_event(form).await?;

    let response_data = EventServiceCreateResponse {
        event_id: event_id.into(),
    };
    Ok(Json(response_data))
}
