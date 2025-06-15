use aide::axum::ApiRouter;
use aide::axum::routing::post_with;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

pub fn router() -> ApiRouter<crate::AppState> {
    ApiRouter::new().api_route_with(
        "/",
        post_with(self::post, |o| o.summary("evops.api.v1.UserService/Create")),
        |r| r.tag(crate::docs::Tag::UserService.into()),
    )
}

async fn post(
    State(state): State<crate::AppState>,
    Json(request): Json<crate::types::UserServiceCreateRequest>,
) -> Result<Json<crate::types::UserServiceCreateResponse>, StatusCode> {
    Ok(Json({
        state
            .create_user(request.try_into().map_err(|_| StatusCode::BAD_REQUEST)?)
            .await
            .into()
    }))
}
