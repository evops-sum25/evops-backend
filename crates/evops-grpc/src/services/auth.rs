use tonic::{Request, Response, Status};

use evops_models::ApiError;

use crate::pb::auth_service_server::{AuthService, AuthServiceServer};
use crate::pb::{
    AuthServiceGetMyInfoRequest, AuthServiceGetMyInfoResponse, AuthServiceLogInRequest,
    AuthServiceLogInResponse, AuthServiceRefreshSessionRequest, AuthServiceRefreshSessionResponse,
    AuthServiceSignUpRequest, AuthServiceSignUpResponse,
};

pub fn server(state: crate::AppState) -> AuthServiceServer<self::Service> {
    AuthServiceServer::new(self::Service { state })
}

pub struct Service {
    state: crate::AppState,
}

#[tonic::async_trait]
impl AuthService for self::Service {
    async fn log_in(
        &self,
        request: Request<AuthServiceLogInRequest>,
    ) -> Result<Response<AuthServiceLogInResponse>, Status> {
        let request_data = request.into_inner();

        let credentials = request_data.credentials.ok_or(ApiError::InvalidArgument(
            "UserServiceLogInRequest.form must not be null".to_owned(),
        ))?;
        let login = {
            evops_models::UserLogin::try_new(credentials.login)
                .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
        };
        let password = {
            evops_models::UserPassword::try_new(credentials.password)
                .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
        };
        let tokens = self.state.log_in(&login, &password).await?;

        let response_data = AuthServiceLogInResponse {
            tokens: Some(tokens.into()),
        };
        Ok(Response::new(response_data))
    }

    async fn get_my_info(
        &self,
        request: Request<AuthServiceGetMyInfoRequest>,
    ) -> Result<Response<AuthServiceGetMyInfoResponse>, Status> {
        let user_id = crate::auth(&self.state, &request)?;
        let user = self.state.get_user_info(user_id).await?;

        let response_data = AuthServiceGetMyInfoResponse {
            user: Some(user.into()),
        };
        Ok(Response::new(response_data))
    }

    async fn refresh_session(
        &self,
        request: Request<AuthServiceRefreshSessionRequest>,
    ) -> Result<Response<AuthServiceRefreshSessionResponse>, Status> {
        let request_data = request.into_inner();

        let refresh_token = evops_models::JsonWebToken::new(request_data.refresh_token);
        let tokens = self.state.refresh_jwt_access(&refresh_token).await?;

        let response_data = AuthServiceRefreshSessionResponse {
            tokens: Some(tokens.into()),
        };
        Ok(Response::new(response_data))
    }

    async fn sign_up(
        &self,
        request: Request<AuthServiceSignUpRequest>,
    ) -> Result<Response<AuthServiceSignUpResponse>, Status> {
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

        let response_data = AuthServiceSignUpResponse {
            tokens: Some(tokens.into()),
        };
        Ok(Response::new(response_data))
    }
}
