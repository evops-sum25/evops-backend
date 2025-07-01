use bytes::{BufMut as _, BytesMut};
use tokio_stream::StreamExt as _;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};
use uuid::Uuid;

use evops_models::ApiError;

use crate::AppState;
use crate::pb::event_service_server::{EventService, EventServiceServer};
use crate::pb::{
    EventServiceCreateRequest, EventServiceCreateResponse, EventServiceFindImageRequest,
    EventServiceFindImageResponse, EventServiceFindRequest, EventServiceFindResponse,
    EventServiceListRequest, EventServiceListResponse, EventServicePushImageRequest,
    EventServicePushImageResponse,
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
            while let Some(message) = request_stream.next().await {
                let message = message?
                    .message
                    .ok_or(ApiError::InvalidArgument(err_msg_not_null.to_owned()))?;
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
        let image_binary = self.state.find_event_image(id).await?;

        let (tx, rx) = tokio::sync::mpsc::channel(4);
        tokio::spawn(async move {
            tx.send(Ok(EventServiceFindImageResponse {
                chunk: image_binary.into(),
            }))
            .await
            .unwrap();
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
