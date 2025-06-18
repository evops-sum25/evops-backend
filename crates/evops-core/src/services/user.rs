use evops_models::{
    CreateUserError, FindUserError, ListUsersError, UserServiceCreateRequest,
    UserServiceCreateResponse, UserServiceFindRequest, UserServiceFindResponse,
    UserServiceListRequest, UserServiceListResponse,
};

impl crate::AppState {
    pub async fn find_user(
        &self,
        request: UserServiceFindRequest,
    ) -> Result<UserServiceFindResponse, FindUserError> {
        let mut db = self.shared_state.db.lock().await;

        Ok(UserServiceFindResponse {
            user: db.find_user(request.id).await?,
        })
    }

    pub async fn list_users(
        &self,
        _request: UserServiceListRequest,
    ) -> Result<UserServiceListResponse, ListUsersError> {
        let mut db = self.shared_state.db.lock().await;

        Ok(UserServiceListResponse {
            users: db.list_users().await?,
        })
    }

    pub async fn create_user(
        &self,
        request: UserServiceCreateRequest,
    ) -> Result<UserServiceCreateResponse, CreateUserError> {
        let mut db = self.shared_state.db.lock().await;

        Ok(UserServiceCreateResponse {
            user: db.create_user(request.form).await?,
        })
    }
}
