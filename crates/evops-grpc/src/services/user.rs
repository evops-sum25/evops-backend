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

        let form = {
            request_data
                .form
                .ok_or_else(|| {
                    ApiError::InvalidArgument({
                        "UserServiceSignUpRequest.form must not be null.".to_owned()
                    })
                })?
                .try_into()?
        };
        let tokens = self.state.sign_up(form).await?;

        let response_data = UserServiceSignUpResponse {
            tokens: Some(tokens.into()),
        };
        Ok(Response::new(response_data))
    }
}
