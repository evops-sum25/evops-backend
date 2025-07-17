use std::pin::Pin;

use aide::OperationOutput;
use aide::axum::ApiRouter;
use aide::axum::routing::get_with;
use aide::transform::{TransformOperation, TransformPathItem};
use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderMap, HeaderValue};
use axum::response::IntoResponse;
use axum_extra::response::FileStream;
use futures::Stream;

use evops_models::ApiResult;

use crate::AppState;
use crate::error::AddResponse as _;
use crate::types::{EventServiceDeleteImageRequestPath, EventServiceFindImageRequestPath};

fn route_docs(r: TransformPathItem) -> TransformPathItem {
    r.tag(crate::docs::Tag::EventService.into())
}
pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new().api_route_with(
        "/",
        get_with(self::get, self::get_docs).delete_with(self::delete, self::delete_docs),
        self::route_docs,
    )
}

fn get_docs(o: TransformOperation) -> TransformOperation {
    o.summary("evops.api.v1.EventService.FindImage")
        .description("Retrieves an event image by ID.")
        .response_bad_request()
        .response_not_found()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn get(
    State(state): State<AppState>,
    Path(path): Path<EventServiceFindImageRequestPath>,
) -> ApiResult<PostResponse> {
    let id = path.image_id.into();
    let image_stream = state.stream_event_image(id).await?;

    let response_stream = FileStream::new(image_stream);
    Ok(PostResponse(response_stream))
}

fn delete_docs(mut o: TransformOperation) -> TransformOperation {
    o.inner_mut().deprecated = true;
    o.summary("evops.api.v1.EventService.DeleteImage")
        .description("Deletes an event image by ID. If there are images to the right, they are shifted one position back.")
        .response_bad_request()
        .response_not_found()
        .response_unprocessable_entity()
        .response_internal_server_error()
}
async fn delete(Path(path): Path<EventServiceDeleteImageRequestPath>) -> ApiResult<()> {
    todo!();
}

const POST_MIME_TYPE: &str = "image/webp";

struct PostResponse(FileStream<Pin<Box<dyn Stream<Item = ApiResult<Bytes>> + Send>>>);
impl IntoResponse for PostResponse {
    fn into_response(self) -> axum::response::Response {
        let mut headers = HeaderMap::with_capacity(1);
        headers.append(CONTENT_TYPE, HeaderValue::from_static(self::POST_MIME_TYPE));
        (headers, self.0).into_response()
    }
}

impl OperationOutput for self::PostResponse {
    type Inner = Bytes;

    fn operation_response(
        _ctx: &mut aide::generate::GenContext,
        _operation: &mut aide::openapi::Operation,
    ) -> Option<aide::openapi::Response> {
        Some(aide::openapi::Response {
            content: [(
                self::POST_MIME_TYPE.to_owned(),
                aide::openapi::MediaType::default(),
            )]
            .into(),
            ..Default::default()
        })
    }

    fn inferred_responses(
        ctx: &mut aide::generate::GenContext,
        operation: &mut aide::openapi::Operation,
    ) -> Vec<(Option<u16>, aide::openapi::Response)> {
        Self::operation_response(ctx, operation)
            .map(|resp| vec![(Some(200), resp)])
            .unwrap_or_default()
    }
}
