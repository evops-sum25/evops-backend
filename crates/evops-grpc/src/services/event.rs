use tonic::{Request, Response, Status};

use evops_pb::api::event_service_server::{EventService, EventServiceServer};

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
        request: Request<evops_pb::api::EventServiceFindRequest>,
    ) -> Result<Response<evops_pb::api::EventServiceFindResponse>, Status> {
        Ok(Response::new({
            self.state
                .find_event(request.into_inner().try_into()?)
                .await
                .map_err(|e| {
                    use evops_models::FindEventError as E;
                    match e {
                        E::NotFound(_) => Status::not_found(e.to_string()),
                        E::Db(_) => Status::internal(e.to_string()),
                    }
                })?
                .into()
        }))
    }

    async fn list(
        &self,
        request: Request<evops_pb::api::EventServiceListRequest>,
    ) -> Result<Response<evops_pb::api::EventServiceListResponse>, Status> {
        Ok(Response::new({
            self.state
                .list_events(request.into_inner().into())
                .await
                .map_err(|e| {
                    use evops_models::ListEventsError as E;
                    match e {
                        E::Db(_) => Status::internal(e.to_string()),
                    }
                })?
                .into()
        }))
    }

    async fn create(
        &self,
        request: Request<evops_pb::api::EventServiceCreateRequest>,
    ) -> Result<Response<evops_pb::api::EventServiceCreateResponse>, Status> {
        Ok(Response::new({
            self.state
                .create_event(request.into_inner().try_into()?)
                .await
                .map_err(|e| {
                    use evops_models::CreateEventError as E;
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
