use aide::axum::ApiRouter;
use aide::axum::routing::post_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::State;

use crate::error::AddResponse as _;

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::UserService.into())
}
pub fn router() -> ApiRouter<crate::AppState> {
    ApiRouter::new().api_route_with(
        "/",
        post_with(self::post, self::post_docs),
        self::route_docs,
    )
}

fn post_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.UserService/Create")
        .response_bad_request()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn post(
    State(state): State<crate::AppState>,
    Json(request): Json<crate::types::UserServiceCreateRequest>,
) -> Result<Json<crate::types::UserServiceCreateResponse>, String> {
    Ok(Json({
        state
            .create_user(request.try_into().map_err(|e: eyre::Error| e.to_string())?)
            .await
            // .map_err(|e| eyre!("{e}"))?
            .map_err(|e| e.to_string())?
            .into()
    }))
}
