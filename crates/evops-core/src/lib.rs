use std::sync::Arc;

use eyre::Context as _;
use url::Url;

pub mod services;

// This private struct only exists to be part of `self::AppState`.
// The reason for its existence is to only wrap the state in an `Arc` once
// rather than wrapping every field (such as `db`).
struct State {
    db: tokio::sync::Mutex<evops_db::Database>,
}

#[derive(Clone)]
pub struct AppState {
    // Here, the `State` struct, defined above, only gets wrapped in an `Arc` once.
    shared_state: Arc<self::State>,
}

impl AppState {
    pub async fn try_new(database_url: &Url) -> eyre::Result<Self> {
        let db = {
            evops_db::Database::establish_connection(database_url)
                .await
                .wrap_err("error connecting to db")?
        };

        Ok(Self {
            shared_state: {
                Arc::new(self::State {
                    db: tokio::sync::Mutex::new(db),
                })
            },
        })
    }

    /// Does the same thing as `self.clone()`,
    /// but the method name explicitly tells that the new object
    /// will point to the same memory location.
    pub fn arc_clone(&self) -> Self {
        Self {
            shared_state: Arc::clone(&self.shared_state),
        }
    }
}
