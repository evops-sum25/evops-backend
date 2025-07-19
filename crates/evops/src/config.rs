use std::time::Duration;

use bytes::Bytes;
use const_format::formatcp;
use eyre::{Context as _, eyre};
use url::Url;

const SERVER_PORT: &str = "SERVER_PORT";
const JWT_ACCESS_SECRET: &str = "JWT_ACCESS_SECRET";
const JWT_REFRESH_SECRET: &str = "JWT_REFRESH_SECRET";
const JWT_ACCESS_EXPIRATION_SECONDS: &str = "JWT_ACCESS_EXPIRATION_SECONDS";
const JWT_REFRESH_EXPIRATION_SECONDS: &str = "JWT_REFRESH_EXPIRATION_SECONDS";
const DATABASE_URL: &str = "DATABASE_URL";
const MINIO_URL: &str = "MINIO_URL";
const MINIO_ROOT_USER: &str = "MINIO_ROOT_USER";
const MINIO_ROOT_PASSWORD: &str = "MINIO_ROOT_PASSWORD";
const ML_SERVER_URL: &str = "ML_SERVER_URL";

pub struct Config {
    pub port: u16,
    pub jwt_access_secret: Bytes,
    pub jwt_refresh_secret: Bytes,
    pub jwt_access_expiration: Duration,
    pub jwt_refresh_expiration: Duration,
    pub database_url: Url,
    pub storage_url: Url,
    pub storage_username: String,
    pub storage_password: String,
    pub ml_server_url: Url,
}

pub fn from_env() -> eyre::Result<self::Config> {
    let port = {
        let raw = std::env::var(SERVER_PORT).wrap_err(SERVER_PORT)?;
        raw.parse::<u16>()
            .wrap_err(formatcp!("variable {SERVER_PORT} is malformed"))?
    };
    let jwt_access_secret = {
        std::env::var_os(JWT_ACCESS_SECRET)
            .ok_or(eyre!(JWT_ACCESS_SECRET))?
            .into_encoded_bytes()
            .into()
    };
    let jwt_refresh_secret = {
        std::env::var_os(JWT_REFRESH_SECRET)
            .ok_or(eyre!(JWT_REFRESH_SECRET))?
            .into_encoded_bytes()
            .into()
    };
    let jwt_access_expiration = {
        let raw =
            std::env::var(JWT_ACCESS_EXPIRATION_SECONDS).wrap_err(JWT_ACCESS_EXPIRATION_SECONDS)?;
        Duration::from_secs(raw.parse().wrap_err(formatcp!(
            "variable {JWT_ACCESS_EXPIRATION_SECONDS} is malformed",
        ))?)
    };
    let jwt_refresh_expiration = {
        let raw = std::env::var(JWT_REFRESH_EXPIRATION_SECONDS)
            .wrap_err(JWT_REFRESH_EXPIRATION_SECONDS)?;
        Duration::from_secs(raw.parse().wrap_err(formatcp!(
            "variable {JWT_REFRESH_EXPIRATION_SECONDS} is malformed",
        ))?)
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
    let ml_server_url = {
        let raw = std::env::var(ML_SERVER_URL).wrap_err(ML_SERVER_URL)?;
        raw.parse::<Url>()
            .wrap_err(formatcp!("variable {ML_SERVER_URL} is malformed"))?
    };

    Ok(self::Config {
        port,
        jwt_access_secret,
        jwt_refresh_secret,
        jwt_access_expiration,
        jwt_refresh_expiration,
        database_url,
        storage_url,
        storage_username,
        storage_password,
        ml_server_url,
    })
}
