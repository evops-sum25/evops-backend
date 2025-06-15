use diesel::ConnectionResult;
use diesel_async::{AsyncConnection as _, AsyncPgConnection};
use url::Url;

mod models;
mod schema;

pub struct Database {
    _connection: AsyncPgConnection,
}

impl Database {
    pub async fn establish_connection(database_url: &Url) -> ConnectionResult<Self> {
        Ok(Self {
            _connection: AsyncPgConnection::establish(database_url.as_str()).await?,
        })
    }
}

// impl Database {
//     pub fn create_event() -> evops_types::EventServiceCreateResponse {}
// }
