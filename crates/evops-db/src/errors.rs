use thiserror::Error;

pub mod prelude {
    pub use super::{CreateTagError, CreateUserError};
}

#[derive(Error, Debug)]
#[error(transparent)]
pub enum CreateUserError {
    Other(#[from] diesel::result::Error),
}

#[derive(Error, Debug)]
pub enum CreateTagError {
    #[error("{0}")]
    Duplicate(String),
    #[error(transparent)]
    Other(#[from] diesel::result::Error),
}
