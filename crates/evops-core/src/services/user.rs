use evops_types::{CreateUserError, UserServiceCreateRequest, UserServiceCreateResponse};

impl crate::AppState {
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
