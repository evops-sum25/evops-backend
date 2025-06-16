use thiserror::Error;

pub mod db {
    pub use evops_db::errors::prelude::*;
}

#[derive(Error, Debug)]
#[error(transparent)]
pub enum CreateTagError {
    Db(#[from] self::db::CreateTagError),
}
