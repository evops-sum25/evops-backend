use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::State;
use axum_extra::extract::Query;
use tap::TryConv as _;

use evops_models::{ApiError, ApiResult};

use crate::error::AddResponse as _;
use crate::types::{
    EventServiceCreateRequest, EventServiceCreateResponse, EventServiceListRequestQuery,
    EventServiceListResponse,
};
use crate::{AppState, Auth};

mod _event_id;
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
        .nest("/{event-id}", self::_event_id::router())
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
    Query(query): Query<EventServiceListRequestQuery>,
) -> ApiResult<Json<EventServiceListResponse>> {
    let search = query.search;
    let last_id = query.last_id.map(Into::into);
    let tags = query.tag_ids.into_iter().map(Into::into).collect();
    let limit = match query.limit {
        Some(lim) => Some({
            lim.try_conv::<evops_models::PgLimit>()
                .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
        }),
        None => None,
    };
    let events = state.list_events(last_id, limit, tags, search).await?;

    let response_data = EventServiceListResponse {
        events: events.into_iter().map(Into::into).collect(),
    };
    Ok(Json(response_data))
}

fn post_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.EventService.Create")
        .description("Creates a new event.")
        .response_bad_request()
        .response_unauthorized()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn post(
    State(state): State<AppState>,
    Auth(user_id): Auth,
    Json(request): Json<EventServiceCreateRequest>,
) -> ApiResult<Json<EventServiceCreateResponse>> {
    let form = request.form.try_into()?;
    let event_id = state.create_event(form, user_id).await?;

    let response_data = EventServiceCreateResponse {
        event_id: event_id.into(),
    };
    Ok(Json(response_data))
}
