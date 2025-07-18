use aide::axum::ApiRouter;
use aide::axum::routing::post_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use schemars::JsonSchema;
use serde::Serialize;

use evops_models::ApiResult;

use crate::AppState;
use crate::error::AddResponse as _;
use crate::types::UserServiceSignUpRequest;

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::UserService.into())
}
pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().api_route_with(
        "/",
        post_with(self::post, self::post_docs),
        self::route_docs,
    )
}

fn post_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.UserService.SignUp")
        .description("...")
        .response_bad_request()
        .response_internal_server_error()
}
async fn post(
    Json(request): Json<UserServiceSignUpRequest>,
) -> ApiResult<Json<self::PostResponse>> {
    todo!();
}

#[derive(Debug, Serialize, JsonSchema)]
struct PostResponse {
    access_token: String,
    token_type: String,
}
