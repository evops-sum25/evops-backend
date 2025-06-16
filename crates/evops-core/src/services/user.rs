use evops_types::{CreateUserRequest, CreateUserResponse};

impl crate::AppState {
    #[must_use]
    pub async fn create_user(
        &self,
        request: CreateUserRequest,
    ) -> Result<CreateUserResponse, evops_db::errors::CreateUserError> {
        let mut db = self.shared_state.db.lock().await;
        Ok(db.create_user(request).await?)
    }
}
