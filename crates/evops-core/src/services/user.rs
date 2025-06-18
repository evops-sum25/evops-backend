use evops_types::{
    CreateUserError, ListUsersError, UserServiceCreateRequest, UserServiceCreateResponse,
    UserServiceListRequest, UserServiceListResponse,
};

impl crate::AppState {
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
