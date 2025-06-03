use std::sync::Arc;

use eyre::Context as _;
use url::Url;

pub mod services;
pub mod types;

pub type AppState = Arc<self::State>;

pub struct State {
    db: tokio::sync::Mutex<evops_db::Database>,
}

impl State {
    pub async fn try_new(database_url: &Url) -> eyre::Result<AppState> {
        let db = {
            evops_db::Database::establish_connection(database_url)
                .await
                .wrap_err("error connecting to db")?
        };

        Ok(Arc::new(Self {
            db: tokio::sync::Mutex::new(db),
        }))
    }
}
