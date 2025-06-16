use aide::axum::ApiRouter;
use aide::axum::routing::post_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

use crate::error::AddResponse as _;

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::EventService.into())
}
pub fn router() -> ApiRouter<crate::AppState> {
    ApiRouter::new().api_route_with(
        "/",
        post_with(self::post, self::post_docs),
        self::route_docs,
    )
}

fn post_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.EventService/Create")
        .response_bad_request()
        .response_unprocessable_entity()
}
async fn post(
    State(state): State<crate::AppState>,
    Json(request): Json<crate::types::EventServiceCreateRequest>,
) -> Result<Json<crate::types::EventServiceCreateResponse>, StatusCode> {
    Ok(Json({
        state
            .create_event(request.try_into().map_err(|_| StatusCode::BAD_REQUEST)?)
            .await
            .into()
    }))
}
