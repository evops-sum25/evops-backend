use tonic::{Request, Response, Status};

use evops_core::AppState;

use crate::pb::event_service_server::{EventService, EventServiceServer};

pub fn server(state: AppState) -> EventServiceServer<self::Service> {
    EventServiceServer::new(self::Service { state })
}

pub struct Service {
    state: AppState,
}

#[tonic::async_trait]
impl EventService for self::Service {
    async fn create(
        &self,
        request: Request<crate::pb::EventServiceCreateRequest>,
    ) -> Result<Response<crate::pb::EventServiceCreateResponse>, Status> {
        Ok(Response::new({
            evops_core::services::event::create(&self.state, request.into_inner().into())
                .await
                .into()
        }))
    }
}

impl From<crate::pb::EventServiceCreateRequest> for evops_core::types::EventServiceCreateRequest {
    fn from(value: crate::pb::EventServiceCreateRequest) -> Self {
        Self {
            name: value.name,
            description: value.description,
        }
    }
}

impl From<evops_core::types::EventServiceCreateResponse> for crate::pb::EventServiceCreateResponse {
    fn from(value: evops_core::types::EventServiceCreateResponse) -> Self {
        Self {
            name: value.name,
            description: value.description,
        }
    }
}
