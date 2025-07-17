use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::{Path, State};

use evops_models::ApiResult;

use crate::AppState;
use crate::error::AddResponse as _;
use crate::types::{UserServiceFindRequestPath, UserServiceFindResponse};

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::UserService.into())
}
pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().api_route_with("/", get_with(self::get, self::get_docs), self::route_docs)
}

fn get_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.UserService.Find")
        .description("Retrieves a user by ID.")
        .response_bad_request()
        .response_not_found()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn get(
    State(state): State<AppState>,
    Path(path): Path<UserServiceFindRequestPath>,
) -> ApiResult<Json<UserServiceFindResponse>> {
    let request = path.user_id.into();
    let found_user = state.find_user(request).await?;

    let response_data = UserServiceFindResponse {
        user: found_user.into(),
    };
    Ok(Json(response_data))
}
