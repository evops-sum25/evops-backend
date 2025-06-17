use tonic::{Request, Response, Status};

use crate::pb::event_service_server::{EventService, EventServiceServer};

pub fn server(state: crate::AppState) -> EventServiceServer<self::Service> {
    EventServiceServer::new(self::Service { state })
}

pub struct Service {
    state: crate::AppState,
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
