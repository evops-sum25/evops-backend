use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::State;

use evops_models::ApiResult;

use crate::error::AddResponse as _;
use crate::types::AuthServiceGetMyInfoResponse;
use crate::{AppState, Auth};

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::AuthService.into())
}
pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().api_route_with("/", get_with(self::get, self::get_docs), self::route_docs)
}

fn get_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.AuthService.GetMyInfo")
        .description("...")
        .response_bad_request()
        .response_unauthorized()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn get(
    State(state): State<AppState>,
    Auth(user_id): Auth,
) -> ApiResult<Json<AuthServiceGetMyInfoResponse>> {
    let user = state.get_user_info(user_id).await?;

    let respone_data = AuthServiceGetMyInfoResponse { user: user.into() };
    Ok(Json(respone_data))
}
