use aide::axum::ApiRouter;
use aide::axum::routing::post_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::State;

use evops_models::ApiResult;

use crate::AppState;
use crate::error::AddResponse as _;
use crate::types::{AuthServiceRefreshSessionRequest, AuthServiceRefreshSessionResponse};

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::AuthService.into())
}
pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().api_route_with(
        "/",
        post_with(self::post, self::post_docs),
        self::route_docs,
    )
}

fn post_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.AuthService.RefreshSession")
        .description("...")
        .response_bad_request()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn post(
    State(state): State<AppState>,
    Json(request): Json<AuthServiceRefreshSessionRequest>,
) -> ApiResult<Json<AuthServiceRefreshSessionResponse>> {
    let refresh_token = request.refresh_token.into();
    let tokens = state.refresh_jwt_access(&refresh_token).await?;

    let response_data = AuthServiceRefreshSessionResponse {
        tokens: tokens.into(),
    };
    Ok(Json(response_data))
}
