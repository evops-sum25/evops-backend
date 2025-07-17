use bytes::{BufMut as _, BytesMut};
use tokio_stream::StreamExt as _;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};
use uuid::Uuid;

use evops_models::ApiError;

use crate::AppState;
use crate::pb::event_service_server::{EventService, EventServiceServer};
use crate::pb::{
    EventServiceCreateRequest, EventServiceCreateResponse, EventServiceDeleteImageRequest,
    EventServiceDeleteImageResponse, EventServiceDeleteRequest, EventServiceDeleteResponse,
    EventServiceFindImageRequest, EventServiceFindImageResponse, EventServiceFindRequest,
    EventServiceFindResponse, EventServiceListRequest, EventServiceListResponse,
    EventServicePushImageRequest, EventServicePushImageResponse, EventServiceReorderImagesRequest,
    EventServiceReorderImagesResponse, EventServiceUpdateRequest, EventServiceUpdateResponse,
};

pub fn server(state: AppState) -> EventServiceServer<self::Service> {
    EventServiceServer::new(self::Service { state })
}

pub struct Service {
    state: AppState,
}

#[tonic::async_trait]
impl EventService for self::Service {
    type FindImageStream = ReceiverStream<Result<EventServiceFindImageResponse, Status>>;

    async fn list(
        &self,
        request: Request<EventServiceListRequest>,
    ) -> Result<Response<EventServiceListResponse>, Status> {
        let request_data = request.into_inner();

        let last_id = match request_data.last_id {
            Some(id) => Some(evops_models::EventId::new({
                id.parse::<Uuid>()
                    .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
            })),
            None => None,
        };
        let limit = match request_data.limit {
            Some(lim) => Some({
                evops_models::PgLimit::try_new(lim)
                    .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
            }),
            None => None,
        };
        let events = self.state.list_events(last_id, limit).await?;

        let response_data = EventServiceListResponse {
            events: events.into_iter().map(Into::into).collect(),
        };
        Ok(Response::new(response_data))
    }

    async fn create(
        &self,
        request: Request<EventServiceCreateRequest>,
    ) -> Result<Response<EventServiceCreateResponse>, Status> {
        let request_data = request.into_inner();

        let form = {
            request_data
                .form
                .ok_or(ApiError::InvalidArgument({
                    "EventServiceCreateRequest.form must not be null.".to_owned()
                }))?
                .try_into()?
        };
        let event_id = self.state.create_event(form).await?;

        let response_data = EventServiceCreateResponse {
            event_id: event_id.to_string(),
        };
        Ok(Response::new(response_data))
    }

    async fn find(
        &self,
        request: Request<EventServiceFindRequest>,
    ) -> Result<Response<EventServiceFindResponse>, Status> {
        let request_data = request.into_inner();

        let id = evops_models::EventId::new({
            request_data
                .id
                .parse::<Uuid>()
                .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
        });
        let found_event = self.state.find_event(id).await?;

        let response_data = EventServiceFindResponse {
            event: Some(found_event.into()),
        };
        Ok(Response::new(response_data))
    }

    async fn update(
        &self,
        request: Request<EventServiceUpdateRequest>,
    ) -> Result<Response<EventServiceUpdateResponse>, Status> {
        let request_data = request.into_inner();

        let event_id = evops_models::EventId::new({
            request_data
                .event_id
                .parse::<Uuid>()
                .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
        });
        let form = {
            request_data
                .form
                .ok_or_else(|| {
                    ApiError::InvalidArgument({
                        let err_msg = "EventServiceUpdateRequest.form must not be null.";
                        err_msg.to_owned()
                    })
                })?
                .try_into()?
        };
        self.state.update_event(event_id, form).await?;

        Ok(Response::new(EventServiceUpdateResponse {}))
    }

    async fn delete(
        &self,
        request: Request<EventServiceDeleteRequest>,
    ) -> Result<Response<EventServiceDeleteResponse>, Status> {
        let request_data = request.into_inner();

        let event_id = evops_models::EventId::new({
            request_data
                .event_id
                .parse::<Uuid>()
                .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
        });
        self.state.delete_event(event_id).await?;

        Ok(Response::new(EventServiceDeleteResponse {}))
    }

    async fn reorder_images(
        &self,
        request: Request<EventServiceReorderImagesRequest>,
    ) -> Result<Response<EventServiceReorderImagesResponse>, Status> {
        todo!();
    }

    async fn push_image(
        &self,
        request: Request<Streaming<EventServicePushImageRequest>>,
    ) -> Result<Response<EventServicePushImageResponse>, Status> {
        use crate::pb::event_service_push_image_request::Message;

        let mut request_stream = request.into_inner();

        let err_msg_not_null = "EventServicePushImageRequest.message must not be null.";

        let event_id = {
            let err_msg_1 = "The first message must contain metadata.";
            let message = {
                request_stream
                    .next()
                    .await
                    .ok_or(ApiError::InvalidArgument(err_msg_1.to_owned()))??
                    .message
                    .ok_or(ApiError::InvalidArgument(err_msg_not_null.to_owned()))?
            };
            let metadata = match message {
                Message::Metadata(data) => data,
                Message::Chunk(_) => {
                    return Err(ApiError::InvalidArgument(err_msg_1.to_owned()).into());
                }
            };

            evops_models::EventId::new({
                metadata
                    .event_id
                    .parse::<Uuid>()
                    .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
            })
        };
        let image = {
            let mut buffer = BytesMut::new();
            while let Some(message) = request_stream.message().await? {
                let message = {
                    message
                        .message
                        .ok_or(ApiError::InvalidArgument(err_msg_not_null.to_owned()))?
                };
                let chunk = match message {
                    Message::Metadata(_) => {
                        let err_msg = "Expected chunk, found metadata. Only the first message needs to contain metadata.";
                        return Err(ApiError::InvalidArgument(err_msg.to_owned()).into());
                    }
                    Message::Chunk(c) => c,
                };
                buffer.put(&*chunk);
            }
            let buffer = buffer.freeze();

            evops_models::EventImage::try_from(buffer)?
        };

        let image_id = self.state.push_event_image(event_id, image).await?;
        let response_data = EventServicePushImageResponse {
            image_id: image_id.to_string(),
        };

        Ok(Response::new(response_data))
    }

    async fn find_image(
        &self,
        request: Request<EventServiceFindImageRequest>,
    ) -> Result<Response<Self::FindImageStream>, Status> {
        let request_data = request.into_inner();

        let id = evops_models::EventImageId::new({
            request_data
                .image_id
                .parse::<Uuid>()
                .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
        });
        let mut image_stream = self.state.stream_event_image(id).await?;

        let (tx, rx) = tokio::sync::mpsc::channel(128);
        let chunk_size = bytesize::kib(40_u64).try_into().unwrap();
        tokio::spawn(async move {
            while let Some(storage_chunk_result) = image_stream.next().await {
                let storage_chunk = match storage_chunk_result {
                    Ok(chunk) => chunk,
                    Err(e) => {
                        _ = tx.send(Err(e.into())).await;
                        return;
                    }
                };
                for chunk in storage_chunk.chunks(chunk_size) {
                    let message = EventServiceFindImageResponse {
                        chunk: chunk.to_vec(),
                    };
                    if tx.send(Ok(message)).await.is_err() {
                        return;
                    }
                }
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn delete_image(
        &self,
        request: Request<EventServiceDeleteImageRequest>,
    ) -> Result<Response<EventServiceDeleteImageResponse>, Status> {
        let request_data = request.into_inner();

        let image_id = evops_models::EventImageId::new({
            request_data
                .image_id
                .parse::<Uuid>()
                .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
        });

        todo!();
    }
}
