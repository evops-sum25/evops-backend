use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::State;

use crate::error::AddResponse as _;

mod create;

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::UserService.into())
}
pub fn router() -> ApiRouter<crate::AppState> {
    ApiRouter::new()
        .api_route_with("/", get_with(self::get, self::get_docs), self::route_docs)
        .nest("/create", self::create::router())
}

fn get_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.UserService/List")
        .response_unprocessable_entity()
        .response_internal_server_error()
}

async fn get(
    State(state): State<crate::AppState>,
    Json(request): Json<crate::types::UserServiceListRequest>,
) -> crate::Result<Json<crate::types::UserServiceListResponse>> {
    Ok(Json({
        state
            .list_users(request.into())
            .await
            .map_err(|e| {
                use evops_models::ListUsersError as E;
                match e {
                    E::Db(e) => crate::error::InternalServerError::from(e.to_string()),
                }
            })?
            .into()
    }))
}
