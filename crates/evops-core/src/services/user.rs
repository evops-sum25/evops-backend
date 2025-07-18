use chrono::Utc;
use evops_models::{
    ApiError, ApiResult, JwtClaims, JwtTokenInfo, JwtTokenType, NewUserForm, User, UserId,
};
use uuid::Uuid;

impl crate::AppState {
    pub async fn list_users(&self) -> ApiResult<Vec<User>> {
        let users = {
            let mut db = self.shared_state.db.lock().await;
            db.list_users().await
        }?;
        Ok(users)
    }

    #[deprecated]
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

    pub async fn signup_user(&self, form: NewUserForm) -> ApiResult<JwtTokenInfo> {
        let user_id = UserId::new(Uuid::now_v7());

        let token = {
            jsonwebtoken::encode(
                &jsonwebtoken::Header::default(),
                &JwtClaims {
                    sub: user_id,
                    exp: Utc::now(),
                },
                &jsonwebtoken::EncodingKey::from_secret(&self.shared_state.jwt_secret),
            )
            .map_err(|e| ApiError::Other(e.to_string()))?
        };

        let token_info = JwtTokenInfo {
            token,
            token_type: JwtTokenType::Bearer,
        };

        Ok(token_info)
    }
}
