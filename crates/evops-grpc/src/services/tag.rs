use tonic::{Request, Response, Status};

use crate::pb::tag_service_server::{TagService, TagServiceServer};

pub fn server(state: crate::AppState) -> TagServiceServer<self::Service> {
    TagServiceServer::new(self::Service { state })
}

pub struct Service {
    state: crate::AppState,
}

#[tonic::async_trait]
impl TagService for self::Service {
    async fn create(
        &self,
        request: Request<crate::pb::TagServiceCreateRequest>,
    ) -> Result<Response<crate::pb::TagServiceCreateResponse>, Status> {
        Ok(Response::new({
            self.state
                .create_tag(request.into_inner().try_into()?)
                .await
                .map_err(|e| {
                    use evops_types::CreateTagError as E;
                    match e {
                        E::AlreadyExists(_) => Status::already_exists(e.to_string()),
                        E::Db(_) => Status::internal(e.to_string()),
                    }
                })?
                .into()
        }))
    }
}
