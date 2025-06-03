use aide::axum::ApiRouter;
use aide::axum::routing::post_with;
use axum::Json;
use axum::extract::State;

use evops_core::AppState;
use evops_core::types::{EventServiceCreateRequest, EventServiceCreateResponse};

use crate::docs::Tag;

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().api_route_with(
        "/",
        post_with(self::post, |o| {
            o.summary("evops.api.v1.EventService/Create")
        }),
        |r| r.tag(Tag::EventService.into()),
    )
}

async fn post(
    State(state): State<AppState>,
    Json(request): Json<EventServiceCreateRequest>,
) -> Json<EventServiceCreateResponse> {
    Json(evops_core::services::event::create(&state, request).await)
}
