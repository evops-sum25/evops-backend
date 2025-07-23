# `evops-grpc`

## Overview

Provides gRPC API endpoints using `tonic` framework. Handles protocol buffer serialization, gRPC-web compatibility, and service implementations. Organized into:
- Protocol buffer definitions (`pb`)
- Service implementations (`services`)
- gRPC-specific utilities (`headers`)

### Service Example

Event Service (`services/event.rs`)
- Creating an event:
```rust
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
```

- Streaming image download:
```rust
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
```

### Key Files

- `lib.rs`: Main entry point
    - Creates gRPC router with `axum`
    - Registers all services
    - Sets up gRPC reflection service for debugging
    - Configures CORS with gRPC-specific headers

- `pb.rs`: PContains auto-generated Protobuf bindings
    - Contains all gRPC service and message definitions
    - Includes file descriptor set for reflection
    - Manual validation implementations

 - `pb/impls.rs`: Protobuf ↔ Domain types
    - Handles validation and type conversions
    - Key responsibilities
        - String to UUID conversion
        - Field validation
        - Model to gRPC response transformations

- `services.rs`: Service Registry
Aggregates all service implementations
