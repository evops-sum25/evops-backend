use std::time::Duration;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHasher as _, SaltString};
use argon2::{Argon2, PasswordVerifier as _};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use evops_models::{
    ApiError, ApiResult, AuthTokens, JsonWebToken, JsonWebTokenHash, JwtClaims, NewUserForm,
    UserId, UserLogin, UserPassword, UserPasswordHash,
};

impl crate::AppState {
    pub async fn sign_up(&self, form: NewUserForm) -> ApiResult<AuthTokens> {
        let user_id = UserId::new(Uuid::now_v7());
        let now = Utc::now();

        let tokens = AuthTokens {
            access: self::generate_jwt(
                user_id,
                &self.shared_state.jwt_access_secret,
                now,
                self.shared_state.jwt_access_expiration,
            )?,
            refresh: self::generate_jwt(
                user_id,
                &self.shared_state.jwt_refresh_secret,
                now,
                self.shared_state.jwt_refresh_expiration,
            )?,
        };
        let refresh_token_hash = self::hash_jwt(&tokens.refresh);
        let password_hash = self::hash_password(&form.password)?;

        {
            let mut db = self.shared_state.db.lock().await;
            db.sign_up(
                user_id,
                &form.login,
                &password_hash,
                &form.display_name,
                &refresh_token_hash,
            )
            .await
        }?;

        Ok(tokens)
    }

    pub async fn log_in(
        &self,
        login: &UserLogin,
        password: &UserPassword,
    ) -> ApiResult<AuthTokens> {
        let (user_id, password_hash) = {
            let mut db = self.shared_state.db.lock().await;
            db.get_password_hash(login).await
        }?;
        self::verify_password(password, &password_hash)?;
        let now = Utc::now();
        let tokens = AuthTokens {
            access: self::generate_jwt(
                user_id,
                &self.shared_state.jwt_access_secret,
                now,
                self.shared_state.jwt_access_expiration,
            )?,
            refresh: self::generate_jwt(
                user_id,
                &self.shared_state.jwt_refresh_secret,
                now,
                self.shared_state.jwt_refresh_expiration,
            )?,
        };
        let token_hash = self::hash_jwt(&tokens.refresh);
        {
            let mut db = self.shared_state.db.lock().await;
            db.insert_refresh_token(&token_hash, user_id).await
        }?;
        Ok(tokens)
    }

    pub async fn refresh_jwt_access(&self, refresh_token: &JsonWebToken) -> ApiResult<AuthTokens> {
        let user_id = self.decode_jwt(refresh_token)?;
        let token_hash = self::hash_jwt(refresh_token);
        {
            let mut db = self.shared_state.db.lock().await;
            db.check_refresh_token(&token_hash).await
        }?;

        let now = Utc::now();
        let tokens = AuthTokens {
            access: self::generate_jwt(
                user_id,
                &self.shared_state.jwt_access_secret,
                now,
                self.shared_state.jwt_access_expiration,
            )?,
            refresh: self::generate_jwt(
                user_id,
                &self.shared_state.jwt_refresh_secret,
                now,
                self.shared_state.jwt_refresh_expiration,
            )?,
        };
        let token_hash = self::hash_jwt(&tokens.refresh);
        {
            let mut db = self.shared_state.db.lock().await;
            db.insert_refresh_token(&token_hash, user_id).await
        }?;
        Ok(tokens)
    }

    pub fn decode_jwt(&self, token: &JsonWebToken) -> ApiResult<UserId> {
        let token_data = {
            jsonwebtoken::decode::<JwtClaims>(
                token.as_ref(),
                &jsonwebtoken::DecodingKey::from_secret(&self.shared_state.jwt_access_secret),
                &jsonwebtoken::Validation::default(),
            )
            .map_err(|e| ApiError::Auth(format!("Failed to parse JWT access token: {e}")))?
        };
        let claims = token_data.claims;
        if claims.exp < Utc::now() {
            return Err(ApiError::Auth("JWT access token expired.".to_owned()));
        }
        Ok(claims.sub)
    }
}

fn hash_password(password: &UserPassword) -> ApiResult<UserPasswordHash> {
    Argon2::default()
        .hash_password(
            password.as_ref().as_bytes(),
            &SaltString::generate(&mut OsRng),
        )
        .map(|password_hash| UserPasswordHash::new(password_hash.to_string()))
        .map_err(|e| ApiError::Other(e.to_string()))
}

fn verify_password(
    new_password: &UserPassword,
    true_password_hash: &UserPasswordHash,
) -> ApiResult<()> {
    let parsed_hash = {
        argon2::PasswordHash::new(true_password_hash.as_ref())
            .map_err(|e| ApiError::Other(e.to_string()))?
    };
    Argon2::default()
        .verify_password(new_password.as_ref().as_bytes(), &parsed_hash)
        .map_err(|_| ApiError::Forbidden("Wrong credentials.".to_owned()))
}

fn generate_jwt(
    user_id: UserId,
    secret: &[u8],
    now: DateTime<Utc>,
    exp: Duration,
) -> ApiResult<JsonWebToken> {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &JwtClaims {
            sub: user_id,
            exp: now + exp,
        },
        &jsonwebtoken::EncodingKey::from_secret(secret),
    )
    .map(JsonWebToken::new)
    .map_err(|e| ApiError::Other(e.to_string()))
}

fn hash_jwt(token: &JsonWebToken) -> JsonWebTokenHash {
    JsonWebTokenHash::new({
        blake3::hash(token.as_ref().as_bytes())
            .as_bytes()
            .to_owned()
    })
}
