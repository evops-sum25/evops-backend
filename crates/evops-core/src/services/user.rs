use evops_types::{CreateUserRequest, CreateUserResponse};

impl crate::AppState {
    #[must_use]
    pub async fn create_user(&self, mut request: CreateUserRequest) -> CreateUserResponse {
        {
            _ = self.shared_state.db.lock().await;
        }

        todo!();
    }
}
