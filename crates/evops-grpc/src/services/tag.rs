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
    async fn find(
        &self,
        request: Request<crate::pb::TagServiceFindRequest>,
    ) -> Result<Response<crate::pb::TagServiceFindResponse>, Status> {
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
        request: Request<crate::pb::TagServiceListRequest>,
    ) -> Result<Response<crate::pb::TagServiceListResponse>, Status> {
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
        request: Request<crate::pb::TagServiceCreateRequest>,
    ) -> Result<Response<crate::pb::TagServiceCreateResponse>, Status> {
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
