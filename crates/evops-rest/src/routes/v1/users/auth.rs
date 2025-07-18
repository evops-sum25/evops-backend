use aide::axum::ApiRouter;
use aide::axum::routing::post_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use schemars::JsonSchema;
use serde::Serialize;

use evops_models::ApiResult;

use crate::AppState;
use crate::error::AddResponse as _;
use crate::types::UserServiceAuthRequest;

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
    o.summary("...")
        .description("...")
        .response_bad_request()
        .response_internal_server_error()
}
async fn post(
    Json(credentials): Json<UserServiceAuthRequest>,
) -> ApiResult<Json<self::PostResponse>> {
    todo!();
}

#[derive(Debug, Serialize, JsonSchema)]
struct PostResponse {
    access_token: String,
    token_type: String,
}
