use nutype::nutype;
use thiserror::Error;
use url::Url;
use uuid::Uuid;

#[derive(Debug)]
pub struct UserServiceCreateRequest {
    pub form: crate::UserForm,
}

#[derive(Debug)]
pub struct UserServiceCreateResponse {
    pub user: crate::User,
}

#[derive(Debug)]
pub struct UserForm {
    pub name: crate::UserName,
    pub profile_picture_url: Option<Url>,
}

#[derive(Debug)]
pub struct User {
    pub id: crate::UserId,
    pub name: crate::UserName,
    pub profile_picture_url: Option<Url>,
}

#[derive(Error, Debug)]
#[error(transparent)]
pub enum CreateUserError {
    Db(eyre::Error),
}

#[nutype(derive(Debug, PartialEq, Eq, Hash))]
pub struct UserId(Uuid);

pub const USER_NAME_MIN_LEN: usize = 1;
pub const USER_NAME_MAX_LEN: usize = 64;
#[nutype(
    new_unchecked,
    validate(len_char_min = crate::USER_NAME_MIN_LEN, len_char_max = crate::USER_NAME_MAX_LEN),
    derive(Debug, PartialEq, Eq, AsRef, Hash),
)]
pub struct UserName(String);
