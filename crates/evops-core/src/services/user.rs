use evops_models::{
    ApiResult, UserServiceCreateRequest, UserServiceCreateResponse, UserServiceFindRequest,
    UserServiceFindResponse, UserServiceListRequest, UserServiceListResponse,
};

impl crate::AppState {
    pub async fn find_user(
        &self,
        request: UserServiceFindRequest,
    ) -> ApiResult<UserServiceFindResponse> {
        let user = {
            let mut db = self.shared_state.db.lock().await;
            db.find_user(request.id).await
        }?;
        Ok(UserServiceFindResponse { user })
    }

    pub async fn list_users(
        &self,
        _request: UserServiceListRequest,
    ) -> ApiResult<UserServiceListResponse> {
        let users = {
            let mut db = self.shared_state.db.lock().await;
            db.list_users().await
        }?;
        Ok(UserServiceListResponse { users })
    }

    pub async fn create_user(
        &self,
        request: UserServiceCreateRequest,
    ) -> ApiResult<UserServiceCreateResponse> {
        let user = {
            let mut db = self.shared_state.db.lock().await;
            db.create_user(request.form).await
        }?;
        Ok(UserServiceCreateResponse { user })
    }
}
