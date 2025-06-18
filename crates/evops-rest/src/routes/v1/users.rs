use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use aide::transform::TransformOperation;
use axum::{Json, extract::State};

use crate::error::AddResponse as _;

mod create;

pub fn router() -> ApiRouter<crate::AppState> {
    ApiRouter::new()
        .api_route("/", get_with(self::get, self::get_docs))
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
                use evops_types::ListUsersError as E;
                match e {
                    E::Db(e) => crate::error::InternalServerError::from(e.to_string()),
                }
            })?
            .into()
    }))
}
