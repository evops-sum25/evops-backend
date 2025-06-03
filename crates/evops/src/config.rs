use const_format::formatcp;
use eyre::Context as _;
use url::Url;

const SERVER_PORT: &str = "SERVER_PORT";
const DATABASE_URL: &str = "DATABASE_URL";

pub struct Config {
    pub port: u16,
    pub database_url: Url,
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

    Ok(self::Config { port, database_url })
}
