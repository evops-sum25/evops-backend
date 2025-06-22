use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::{Query, State};

use evops_models::ApiResult;

use crate::error::AddResponse as _;

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::UserService.into())
}
pub fn router() -> ApiRouter<crate::AppState> {
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
    State(state): State<crate::AppState>,
    Query(request): Query<crate::types::UserServiceFindRequest>,
) -> ApiResult<Json<crate::types::UserServiceFindResponse>> {
    Ok(Json(state.find_user(request.into()).await?.into()))
}
