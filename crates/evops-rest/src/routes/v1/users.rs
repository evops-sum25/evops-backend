use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::State;

use evops_models::ApiResult;

use crate::AppState;
use crate::error::AddResponse as _;
use crate::types::UserServiceListResponse;

mod _user_id;
mod login;
mod signup;

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::UserService.into())
}
pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .api_route_with("/", get_with(self::get, self::get_docs), self::route_docs)
        .nest("/{user-id}", self::_user_id::router())
        .nest("/login", self::login::router())
        .nest("/signup", self::signup::router())
}

fn get_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.UserService.List")
        .description("Lists all users.")
        .response_bad_request()
        .response_unprocessable_entity()
        .response_internal_server_error()
}

async fn get(State(state): State<AppState>) -> ApiResult<Json<UserServiceListResponse>> {
    let users = state.list_users().await?;

    let response_data = UserServiceListResponse {
        users: users.into_iter().map(Into::into).collect(),
    };
    Ok(Json(response_data))
}
