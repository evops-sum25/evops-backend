use aide::axum::ApiRouter;
use aide::axum::routing::post_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::State;

use evops_models::ApiResult;

use crate::AppState;
use crate::error::AddResponse as _;
use crate::types::{AuthServiceLogInRequest, AuthServiceLogInResponse};

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
    o.summary("evops.api.v1.AuthService.LogIn")
        .description("...")
        .response_bad_request()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn post(
    State(state): State<AppState>,
    Json(request): Json<AuthServiceLogInRequest>,
) -> ApiResult<Json<AuthServiceLogInResponse>> {
    let login = request.credentials.login.try_into()?;
    let password = request.credentials.password.try_into()?;
    let tokens = state.log_in(&login, &password).await?;

    let response_data = AuthServiceLogInResponse {
        tokens: tokens.into(),
    };
    Ok(Json(response_data))
}
