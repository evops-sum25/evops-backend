use evops_types::{CreateUserError, CreateUserRequest, CreateUserResponse};

impl crate::AppState {
    pub async fn create_user(
        &self,
        request: CreateUserRequest,
    ) -> Result<CreateUserResponse, CreateUserError> {
        let mut db = self.shared_state.db.lock().await;

        db.create_user(request).await
    }
}
