use tonic::{Request, Response, Status};

use crate::pb::user_service_server::{UserService, UserServiceServer};

pub fn server(state: crate::AppState) -> UserServiceServer<self::Service> {
    UserServiceServer::new(self::Service { state })
}

pub struct Service {
    state: crate::AppState,
}

#[tonic::async_trait]
impl UserService for self::Service {
    async fn create(
        &self,
        request: Request<crate::pb::UserServiceCreateRequest>,
    ) -> Result<Response<crate::pb::UserServiceCreateResponse>, Status> {
        todo!();
    }
}
