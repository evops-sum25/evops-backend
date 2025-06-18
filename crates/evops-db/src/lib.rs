use diesel::{Connection, PgConnection};
use diesel_async::{AsyncConnection as _, AsyncPgConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use eyre::{Context, eyre};
use tracing::debug;
use url::Url;

mod models;
mod schema;
mod services;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub struct Database {
    conn: AsyncPgConnection,
}

impl Database {
    fn run_migrations(
        connection: &mut impl MigrationHarness<diesel::pg::Pg>,
    ) -> diesel::migration::Result<()> {
        debug!("running database migrations");
        connection.run_pending_migrations(MIGRATIONS)?;
        Ok(())
    }

    pub async fn establish_connection(database_url: &Url) -> eyre::Result<Self> {
        Self::run_migrations(&mut PgConnection::establish(database_url.as_str())?)
            .map_err(|e| eyre!("{e}"))
            .wrap_err("failed to run migrations")?;

        Ok(Self {
            conn: AsyncPgConnection::establish(database_url.as_str()).await?,
        })
    }
}
