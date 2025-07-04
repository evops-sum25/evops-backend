use evops_models::{ApiResult, NewUserForm, User, UserId};

impl crate::AppState {
    pub async fn list_users(&self) -> ApiResult<Vec<User>> {
        let users = {
            let mut db = self.shared_state.db.lock().await;
            db.list_users().await
        }?;
        Ok(users)
    }

    pub async fn create_user(&self, request: NewUserForm) -> ApiResult<UserId> {
        let user_id = {
            let mut db = self.shared_state.db.lock().await;
            db.create_user(request).await
        }?;
        Ok(user_id)
    }

    pub async fn find_user(&self, id: UserId) -> ApiResult<User> {
        let user = {
            let mut db = self.shared_state.db.lock().await;
            db.find_user(id).await
        }?;
        Ok(user)
    }
}
