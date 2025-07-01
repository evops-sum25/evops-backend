use const_format::formatcp;
use eyre::Context as _;
use url::Url;

const SERVER_PORT: &str = "SERVER_PORT";
const DATABASE_URL: &str = "DATABASE_URL";
const MINIO_URL: &str = "MINIO_URL";
const MINIO_ROOT_USER: &str = "MINIO_ROOT_USER";
const MINIO_ROOT_PASSWORD: &str = "MINIO_ROOT_PASSWORD";

pub struct Config {
    pub port: u16,
    pub database_url: Url,
    pub storage_url: Url,
    pub storage_username: String,
    pub storage_password: String,
}

pub fn from_env() -> eyre::Result<self::Config> {
    let port = {
        let raw = std::env::var(SERVER_PORT).wrap_err(SERVER_PORT)?;
        raw.parse::<u16>()
            .wrap_err(formatcp!("variable {SERVER_PORT} is malformed"))?
    };
    let database_url = {
        let raw = std::env::var(DATABASE_URL).wrap_err(DATABASE_URL)?;
        raw.parse::<Url>()
            .wrap_err(formatcp!("variable {DATABASE_URL} is malformed"))?
    };
    let storage_url = {
        let raw = std::env::var(MINIO_URL).wrap_err(MINIO_URL)?;
        raw.parse::<Url>()
            .wrap_err(formatcp!("variable {MINIO_URL} is malformed"))?
    };
    let storage_username = std::env::var(MINIO_ROOT_USER).wrap_err(MINIO_ROOT_USER)?;
    let storage_password = std::env::var(MINIO_ROOT_PASSWORD).wrap_err(MINIO_ROOT_PASSWORD)?;

    Ok(self::Config {
        port,
        database_url,
        storage_url,
        storage_username,
        storage_password,
    })
}
