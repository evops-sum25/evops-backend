use std::time::Duration;

use chrono::{DateTime, Utc};
use evops_models::{ApiError, ApiResult, JwtClaims, JwtTokenType, NewUserForm, User, UserId};
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
            self::generate_jwt_access_token(
                user_id,
                now,
                &self.shared_state.jwt_access_secret,
                self.shared_state.jwt_access_expiration,
            )?
        };
        let refresh_token = {
            self::generate_jwt_refresh_token(
                user_id,
                now,
                &self.shared_state.jwt_refresh_secret,
                self.shared_state.jwt_refresh_expiration,
            )?
        };

        todo!()
    }
}

fn generate_jwt_access_token(
    user_id: UserId,
    now: DateTime<Utc>,
    secret: &[u8],
    exp: Duration,
) -> ApiResult<String> {
    self::generate_jwt_token(
        &JwtClaims {
            sub: user_id,
            iat: now,
            exp: now + exp,
            token_type: JwtTokenType::Access,
        },
        secret,
    )
}

fn generate_jwt_refresh_token(
    user_id: UserId,
    now: DateTime<Utc>,
    secret: &[u8],
    exp: Duration,
) -> ApiResult<String> {
    self::generate_jwt_token(
        &JwtClaims {
            sub: user_id,
            iat: now,
            exp: now + exp,
            token_type: JwtTokenType::Refresh,
        },
        secret,
    )
}

fn generate_jwt_token(claims: &JwtClaims, secret: &[u8]) -> ApiResult<String> {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        claims,
        &jsonwebtoken::EncodingKey::from_secret(secret),
    )
    .map_err(|e| ApiError::Other(e.to_string()))
}
