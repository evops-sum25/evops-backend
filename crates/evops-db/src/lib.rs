use diesel::ConnectionResult;
use diesel_async::{AsyncConnection as _, AsyncPgConnection};
use url::Url;

mod models;
mod schema;
mod services;

pub struct Database {
    conn: AsyncPgConnection,
}

impl Database {
    pub async fn establish_connection(database_url: &Url) -> ConnectionResult<Self> {
        Ok(Self {
            conn: AsyncPgConnection::establish(database_url.as_str()).await?,
        })
    }
}
