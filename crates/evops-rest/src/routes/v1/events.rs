use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::Json;
use axum::extract::State;

use crate::error::AddResponse as _;

mod _id;
mod create;

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::EventService.into())
}
pub fn router() -> ApiRouter<crate::AppState> {
    ApiRouter::new()
        .api_route_with("/", get_with(self::get, self::get_docs), self::route_docs)
        .nest("/{id}", self::_id::router())
        .nest("/create", self::create::router())
}

fn get_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.EventService/List")
        .response_bad_request() // TODO:
        .response_unprocessable_entity()
        .response_internal_server_error()
}

async fn get(
    State(state): State<crate::AppState>,
    // Query(request): Query<crate::types::EventServiceListRequest>,
) -> crate::Result<Json<crate::types::EventServiceListResponse>> {
    Ok(Json({
        state
            .list_events(crate::types::EventServiceListRequest.into())
            .await?
            .into()
    }))
}
