use std::sync::Arc;
use std::time::Duration;

use bon::bon;
use bytes::Bytes;
use eyre::Context as _;
use url::Url;

mod services;

// This private struct only exists to be part of `self::AppState`.
// The reason for its existence is to only wrap the state in an `Arc` once
// rather than wrapping every field (such as `db`).
struct State {
    jwt_access_secret: Bytes,
    jwt_refresh_secret: Bytes,
    jwt_access_expiration: Duration,
    jwt_refresh_expiration: Duration,
    db: tokio::sync::Mutex<evops_db::Database>,
    storage: evops_storage::Storage,
    ml_client: tokio::sync::Mutex<evops_ml_client::MlClient>,
}

#[derive(Clone)]
pub struct AppState {
    // Here, the `State` struct, defined above, only gets wrapped in an `Arc` once.
    shared_state: Arc<self::State>,
}

#[bon]
impl AppState {
    #[builder]
    pub async fn new(
        jwt_access_secret: Bytes,
        jwt_refresh_secret: Bytes,
        jwt_access_expiration: Duration,
        jwt_refresh_expiration: Duration,
        database_url: &Url,
        storage_url: &Url,
        storage_username: &str,
        storage_password: &str,
        ml_client_url: &Url,
    ) -> eyre::Result<Self> {
        let db = {
            evops_db::Database::establish_connection(database_url)
                .await
                .wrap_err("error connecting to db")?
        };
        let storage = {
            evops_storage::Storage::builder()
                .base_url(storage_url)
                .username(storage_username)
                .password(storage_password)
                .build()
                .await
                .wrap_err("error connecting to file storage")?
        };
        let ml_client = {
            evops_ml_client::MlClient::new(ml_client_url)
                .await
                .wrap_err("error connecting to ml server")?
        };
        Ok(Self {
            shared_state: {
                Arc::new(self::State {
                    jwt_access_secret,
                    jwt_refresh_secret,
                    jwt_access_expiration,
                    jwt_refresh_expiration,
                    db: tokio::sync::Mutex::new(db),
                    storage,
                    ml_client: tokio::sync::Mutex::new(ml_client),
                })
            },
        })
    }

    /// Does the same thing as `self.clone()`,
    /// but the method name explicitly tells that the new object
    /// will point to the same memory location.
    #[must_use]
    pub fn arc_clone(&self) -> Self {
        Self {
            shared_state: Arc::clone(&self.shared_state),
        }
    }
}
