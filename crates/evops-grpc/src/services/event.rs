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
    async fn find(
        &self,
        request: Request<crate::pb::EventServiceFindRequest>,
    ) -> Result<Response<crate::pb::EventServiceFindResponse>, Status> {
        todo!();
    }

    async fn list(
        &self,
        request: Request<crate::pb::EventServiceListRequest>,
    ) -> Result<Response<crate::pb::EventServiceListResponse>, Status> {
        todo!();
    }

    async fn create(
        &self,
        request: Request<crate::pb::EventServiceCreateRequest>,
    ) -> Result<Response<crate::pb::EventServiceCreateResponse>, Status> {
        Ok(Response::new({
            self.state
                .create_event(request.into_inner().try_into()?)
                .await
                .map_err(|e| {
                    use evops_types::CreateEventError as E;
                    match e {
                        E::AuthorNotFound(_) | E::TagNotFound(_) => {
                            Status::not_found(e.to_string())
                        }
                        E::Db(_) => Status::internal(e.to_string()),
                    }
                })?
                .into()
        }))
    }
}
