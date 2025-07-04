use tonic::{Request, Response, Status};
use uuid::Uuid;

use evops_models::ApiError;

use crate::pb::user_service_server::{UserService, UserServiceServer};
use crate::pb::{UserServiceCreateResponse, UserServiceFindResponse, UserServiceListResponse};

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
        request: Request<crate::pb::UserServiceFindRequest>,
    ) -> Result<Response<crate::pb::UserServiceFindResponse>, Status> {
        let request_data = request.into_inner();

        let id = {
            evops_models::UserId::new({
                request_data
                    .id
                    .parse::<Uuid>()
                    .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
            })
        };
        let found_user = self.state.find_user(id).await?;

        let response_data = UserServiceFindResponse {
            user: Some(found_user.into()),
        };
        Ok(Response::new(response_data))
    }

    async fn list(
        &self,
        _request: Request<crate::pb::UserServiceListRequest>,
    ) -> Result<Response<crate::pb::UserServiceListResponse>, Status> {
        let users = self.state.list_users().await?;

        let response_data = UserServiceListResponse {
            users: users.into_iter().map(Into::into).collect(),
        };
        Ok(Response::new(response_data))
    }

    async fn create(
        &self,
        request: Request<crate::pb::UserServiceCreateRequest>,
    ) -> Result<Response<crate::pb::UserServiceCreateResponse>, Status> {
        let request_data = request.into_inner();

        let form = {
            request_data
                .form
                .ok_or({
                    let err_msg = "UserServiceCreateRequest.form must not be null.";
                    ApiError::InvalidArgument(err_msg.to_owned())
                })?
                .try_into()?
        };
        let user_id = self.state.create_user(form).await?;

        let response_data = UserServiceCreateResponse {
            user_id: user_id.to_string(),
        };
        Ok(Response::new(response_data))
    }
}
