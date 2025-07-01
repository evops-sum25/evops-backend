use aide::axum::ApiRouter;
use aide::axum::routing::post_with;
use aide::transform::{TransformOperation, TransformPathItem};
use aide_axum_typed_multipart::TypedMultipart as AideMultipart;
use axum::Json;
use axum::extract::{Path, State};
use axum_typed_multipart::TypedMultipart;

use evops_models::ApiResult;

use crate::AppState;
use crate::error::AddResponse as _;
use crate::types::{
    EventServicePushImageRequestMultipart, EventServicePushImageRequestPath,
    EventServicePushImageResponse,
};

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::EventService.into())
}
pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().api_route_with(
        "/",
        post_with(self::post, self::post_docs),
        self::route_docs,
    )
}

fn post_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.EventService.PushImage")
        .description("Adds a new image to the event with the specified ID.")
        .response_bad_request()
        .response_not_found()
        .response_conflict()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn post(
    State(state): State<AppState>,
    Path(path): Path<EventServicePushImageRequestPath>,
    AideMultipart(TypedMultipart(multipart)): AideMultipart<EventServicePushImageRequestMultipart>,
) -> ApiResult<Json<EventServicePushImageResponse>> {
    let event_id = path.id.into();
    let image = multipart.image.0.contents.try_into()?;
    let image_id = state.push_event_image(event_id, image).await?;

    let response_data = EventServicePushImageResponse {
        image_id: image_id.into(),
    };
    Ok(Json(response_data))
}
