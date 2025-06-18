use nutype::nutype;
use thiserror::Error;
use url::Url;
use uuid::Uuid;

#[derive(Debug)]
pub struct UserServiceFindRequest {
    pub id: crate::UserId,
}

#[derive(Debug)]
pub struct UserServiceFindResponse {
    pub user: crate::User,
}

#[derive(Debug)]
pub struct UserServiceListRequest;

#[derive(Debug)]
pub struct UserServiceListResponse {
    pub users: Vec<crate::User>,
}

#[derive(Debug)]
pub struct UserServiceCreateRequest {
    pub form: crate::NewUserForm,
}

#[derive(Debug)]
pub struct UserServiceCreateResponse {
    pub user: crate::User,
}

#[derive(Debug)]
pub struct NewUserForm {
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
pub enum FindUserError {
    #[error("User with ID {0} was not found.")]
    NotFound(UserId),
    #[error(transparent)]
    Db(#[from] diesel::result::Error),
}

#[derive(Error, Debug)]
pub enum ListUsersError {
    #[error(transparent)]
    Db(#[from] diesel::result::Error),
}

#[derive(Error, Debug)]
pub enum CreateUserError {
    #[error(transparent)]
    Db(#[from] diesel::result::Error),
}

#[nutype(derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display))]
pub struct UserId(Uuid);

pub const USER_NAME_MIN_LEN: usize = 1;
pub const USER_NAME_MAX_LEN: usize = 64;
#[nutype(
    new_unchecked,
    validate(len_char_max = crate::USER_NAME_MAX_LEN, not_empty),
    derive(Debug, PartialEq, Eq, AsRef, Hash),
)]
pub struct UserName(String);
