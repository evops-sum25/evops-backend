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
    async fn find(
        &self,
        request: Request<crate::pb::UserServiceFindRequest>,
    ) -> Result<Response<crate::pb::UserServiceFindResponse>, Status> {
        Ok(Response::new({
            self.state
                .find_user(request.into_inner().try_into()?)
                .await
                .map_err(|e| {
                    use evops_models::FindUserError as E;
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
        request: Request<crate::pb::UserServiceListRequest>,
    ) -> Result<Response<crate::pb::UserServiceListResponse>, Status> {
        Ok(Response::new({
            self.state
                .list_users(request.into_inner().into())
                .await
                .map_err(|e| {
                    use evops_models::ListUsersError as E;
                    match e {
                        E::Db(e) => Status::internal(e.to_string()),
                    }
                })?
                .into()
        }))
    }

    async fn create(
        &self,
        request: Request<crate::pb::UserServiceCreateRequest>,
    ) -> Result<Response<crate::pb::UserServiceCreateResponse>, Status> {
        Ok(Response::new({
            self.state
                .create_user(request.into_inner().try_into()?)
                .await
                .map_err(|e| {
                    use evops_models::CreateUserError as E;
                    match e {
                        E::Db(e) => Status::internal(e.to_string()),
                    }
                })?
                .into()
        }))
    }
}
