use tonic::{Request, Response, Status};

use evops_pb::api::tag_service_server::{TagService, TagServiceServer};

pub fn server(state: crate::AppState) -> TagServiceServer<self::Service> {
    TagServiceServer::new(self::Service { state })
}

pub struct Service {
    state: crate::AppState,
}

#[tonic::async_trait]
impl TagService for self::Service {
    async fn find(
        &self,
        request: Request<evops_pb::api::TagServiceFindRequest>,
    ) -> Result<Response<evops_pb::api::TagServiceFindResponse>, Status> {
        Ok(Response::new({
            self.state
                .find_tag(request.into_inner().try_into()?)
                .await
                .map_err(|e| {
                    use evops_models::FindTagError as E;
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
        request: Request<evops_pb::api::TagServiceListRequest>,
    ) -> Result<Response<evops_pb::api::TagServiceListResponse>, Status> {
        Ok(Response::new({
            self.state
                .list_tags(request.into_inner().into())
                .await
                .map_err(|e| {
                    use evops_models::ListTagsError as E;
                    match e {
                        E::Db(_) => Status::internal(e.to_string()),
                    }
                })?
                .into()
        }))
    }

    async fn create(
        &self,
        request: Request<evops_pb::api::TagServiceCreateRequest>,
    ) -> Result<Response<evops_pb::api::TagServiceCreateResponse>, Status> {
        Ok(Response::new({
            self.state
                .create_tag(request.into_inner().try_into()?)
                .await
                .map_err(|e| {
                    use evops_models::CreateTagError as E;
                    match e {
                        E::AlreadyExists(_) => Status::already_exists(e.to_string()),
                        E::Db(_) => Status::internal(e.to_string()),
                    }
                })?
                .into()
        }))
    }
}
