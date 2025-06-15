use aide::axum::ApiRouter;
use aide::axum::routing::post_with;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

pub fn router() -> ApiRouter<crate::AppState> {
    ApiRouter::new().api_route_with(
        "/",
        post_with(self::post, |o| {
            o.summary("evops.api.v1.EventService/Create")
        }),
        |r| r.tag(crate::docs::Tag::EventService.into()),
    )
}

async fn post(
    State(state): State<crate::AppState>,
    Json(request): Json<crate::types::EventServiceCreateRequest>,
) -> Result<Json<crate::types::EventServiceCreateResponse>, StatusCode> {
    Ok(Json({
        state
            .create_event(request.try_into().map_err(|_| StatusCode::BAD_REQUEST)?)
            .await
            .into()
    }))
}
