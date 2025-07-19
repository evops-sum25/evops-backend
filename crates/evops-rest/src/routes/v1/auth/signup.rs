use aide::axum::ApiRouter;
use aide::axum::routing::post_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::State;

use evops_models::ApiResult;

use crate::AppState;
use crate::error::AddResponse as _;
use crate::types::{AuthServiceSignUpRequest, AuthServiceSignUpResponse};

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
    o.summary("evops.api.v1.AuthService.SignUp")
        .description("...")
        .response_bad_request()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn post(
    State(state): State<AppState>,
    Json(request): Json<AuthServiceSignUpRequest>,
) -> ApiResult<Json<AuthServiceSignUpResponse>> {
    let form = request.form.try_into()?;
    let tokens = state.sign_up(form).await?;

    let response_data = AuthServiceSignUpResponse {
        tokens: tokens.into(),
    };
    Ok(Json(response_data))
}
