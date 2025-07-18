use tonic::{Request, Response, Status};

use evops_models::ApiError;

use crate::pb::user_service_server::{UserService, UserServiceServer};
use crate::pb::{
    UserServiceLogInRequest, UserServiceLogInResponse, UserServiceRefreshRequest,
    UserServiceRefreshResponse, UserServiceSignUpRequest, UserServiceSignUpResponse,
};

pub fn server(state: crate::AppState) -> UserServiceServer<self::Service> {
    UserServiceServer::new(self::Service { state })
}

pub struct Service {
    state: crate::AppState,
}

#[tonic::async_trait]
impl UserService for self::Service {
    async fn log_in(
        &self,
        request: Request<UserServiceLogInRequest>,
    ) -> Result<Response<UserServiceLogInResponse>, Status> {
        let request_data = request.into_inner();
        todo!();
    }

    async fn refresh(
        &self,
        request: Request<UserServiceRefreshRequest>,
    ) -> Result<Response<UserServiceRefreshResponse>, Status> {
        let request_data = request.into_inner();
        todo!();
    }

    async fn sign_up(
        &self,
        request: Request<UserServiceSignUpRequest>,
    ) -> Result<Response<UserServiceSignUpResponse>, Status> {
        let request_data = request.into_inner();
        todo!();
    }
}
