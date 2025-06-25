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
                .await?
                .into()
        }))
    }

    async fn list(
        &self,
        request: Request<crate::pb::TagServiceListRequest>,
    ) -> Result<Response<crate::pb::TagServiceListResponse>, Status> {
        Ok(Response::new({
            self.state
                .list_tags(request.into_inner().try_into()?)
                .await?
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
                .await?
                .into()
        }))
    }
}
