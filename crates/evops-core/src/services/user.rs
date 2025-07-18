use std::time::Duration;

use chrono::{DateTime, Utc};
use evops_models::{ApiError, ApiResult, JsonWebToken, JwtClaims, NewUserForm, User, UserId};
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

    pub async fn signup_user(&self, form: NewUserForm) -> ApiResult<()> {
        let user_id = UserId::new(Uuid::now_v7());
        let now = Utc::now();

        let access_token = {
            self::generate_jwt(
                user_id,
                &self.shared_state.jwt_access_secret,
                now,
                self.shared_state.jwt_access_expiration,
            )?
        };
        let refresh_token = {
            self::generate_jwt(
                user_id,
                &self.shared_state.jwt_refresh_secret,
                now,
                self.shared_state.jwt_refresh_expiration,
            )?
        };

        todo!()
    }

    pub fn decode_jwt(&self, token: &JsonWebToken) -> ApiResult<JwtClaims> {
        let token_data = {
            jsonwebtoken::decode::<JwtClaims>(
                token.as_ref(),
                &jsonwebtoken::DecodingKey::from_secret(&self.shared_state.jwt_access_secret),
                &jsonwebtoken::Validation::default(),
            )
            .map_err(|e| ApiError::Other(e.to_string()))?
        };
        Ok(token_data.claims)
    }
}

fn generate_jwt(
    user_id: UserId,
    secret: &[u8],
    now: DateTime<Utc>,
    exp: Duration,
) -> ApiResult<String> {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &JwtClaims {
            sub: user_id,
            iat: now,
            exp: now + exp,
        },
        &jsonwebtoken::EncodingKey::from_secret(secret),
    )
    .map_err(|e| ApiError::Other(e.to_string()))
}
