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
        request: Request<crate::pb::EventServiceListRequest>,
    ) -> Result<Response<crate::pb::EventServiceListResponse>, Status> {
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
        request: Request<crate::pb::EventServiceCreateRequest>,
    ) -> Result<Response<crate::pb::EventServiceCreateResponse>, Status> {
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
