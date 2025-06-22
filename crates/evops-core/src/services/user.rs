use evops_models::{
    ApiResult, UserServiceCreateRequest, UserServiceCreateResponse, UserServiceFindRequest,
    UserServiceFindResponse, UserServiceListRequest, UserServiceListResponse,
};

impl crate::AppState {
    pub async fn find_user(
        &self,
        request: UserServiceFindRequest,
    ) -> ApiResult<UserServiceFindResponse> {
        let mut db = self.shared_state.db.lock().await;

        Ok(UserServiceFindResponse {
            user: db.find_user(request.id).await?,
        })
    }

    pub async fn list_users(
        &self,
        _request: UserServiceListRequest,
    ) -> ApiResult<UserServiceListResponse> {
        let mut db = self.shared_state.db.lock().await;

        Ok(UserServiceListResponse {
            users: db.list_users().await?,
        })
    }

    pub async fn create_user(
        &self,
        request: UserServiceCreateRequest,
    ) -> ApiResult<UserServiceCreateResponse> {
        let mut db = self.shared_state.db.lock().await;

        Ok(UserServiceCreateResponse {
            user: db.create_user(request.form).await?,
        })
    }
}
