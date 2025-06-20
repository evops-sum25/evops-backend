use tonic::{Request, Response, Status};

use evops_pb::api::user_service_server::{UserService, UserServiceServer};

pub fn server(state: crate::AppState) -> UserServiceServer<self::Service> {
    UserServiceServer::new(self::Service { state })
}

pub struct Service {
    state: crate::AppState,
}

#[tonic::async_trait]
impl UserService for self::Service {
    async fn find(
        &self,
        request: Request<evops_pb::api::UserServiceFindRequest>,
    ) -> Result<Response<evops_pb::api::UserServiceFindResponse>, Status> {
        Ok(Response::new({
            self.state
                .find_user(request.into_inner().try_into()?)
                .await?
                .into()
        }))
    }

    async fn list(
        &self,
        request: Request<evops_pb::api::UserServiceListRequest>,
    ) -> Result<Response<evops_pb::api::UserServiceListResponse>, Status> {
        Ok(Response::new({
            self.state
                .list_users(request.into_inner().into())
                .await?
                .into()
        }))
    }

    async fn create(
        &self,
        request: Request<evops_pb::api::UserServiceCreateRequest>,
    ) -> Result<Response<evops_pb::api::UserServiceCreateResponse>, Status> {
        Ok(Response::new({
            self.state
                .create_user(request.into_inner().try_into()?)
                .await?
                .into()
        }))
    }
}
