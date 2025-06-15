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
            self.state
                .create_event(request.into_inner().into())
                .await
                .into()
        }))
    }
}

impl From<crate::pb::EventServiceCreateRequest> for evops_types::CreateEventRequest {
    fn from(value: crate::pb::EventServiceCreateRequest) -> Self {
        todo!();
    }
}

impl From<evops_types::CreateEventResponse> for crate::pb::EventServiceCreateResponse {
    fn from(value: evops_types::CreateEventResponse) -> Self {
        todo!();
    }
}
