use std::sync::LazyLock;

use chrono::{DateTime, Utc};
use nutype::nutype;
use regex::Regex;
use thiserror::Error;
use url::Url;
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateUserRequest {
    pub name: UserName,
    pub profile_picture_url: Option<Url>,
}

#[derive(Debug)]
pub struct CreateUserResponse {
    pub id: UserId,
    pub name: UserName,
    pub profile_picture_url: Option<Url>,
}

#[derive(Error, Debug)]
#[error(transparent)]
pub enum CreateUserError {
    Db(eyre::Error),
}

#[derive(Debug)]
pub struct CreateEventResponse {
    pub id: EventId,
    pub author: CreateUserResponse,
    pub image_urls: Vec<Url>,
    pub title: EventTitle,
    pub description: EventDescription,
    pub tags: Vec<CreateTagResponse>,
    pub with_attendance: bool,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

#[nutype(derive(Debug, PartialEq, Eq, Hash))]
pub struct EventId(Uuid);

#[nutype(new_unchecked, derive(Debug, PartialEq, Eq, AsRef, Hash))]
pub struct EventTitle(String);

#[nutype(new_unchecked, derive(Debug, PartialEq, Eq, AsRef, Hash))]
pub struct EventDescription(String);

#[derive(Debug)]
pub struct CreateEventRequest {
    pub author_id: UserId,
    pub image_urls: Option<Vec<Url>>,
    pub title: EventTitle,
    pub description: EventDescription,
    pub tag_ids: Option<Vec<TagId>>,
    pub with_attendance: bool,
}

#[derive(Error, Debug)]
#[error(transparent)]
pub enum CreateEventError {
    Db(#[from] eyre::Error),
}

#[nutype(derive(Debug, PartialEq, Eq, Hash))]
pub struct UserId(Uuid);

pub const USER_NAME_MAX_LEN: usize = 64;
#[nutype(
    new_unchecked,
    validate(len_char_max = USER_NAME_MAX_LEN, not_empty),
    derive(Debug, PartialEq, Eq, AsRef, Hash),
)]
pub struct UserName(String);

#[derive(Debug)]
pub struct CreateTagRequest {
    pub name: TagName,
    pub aliases: Option<Vec<TagAlias>>,
}

#[derive(Debug)]
pub struct CreateTagResponse {
    pub id: TagId,
    pub name: TagName,
    pub aliases: Vec<TagAlias>,
}

#[derive(Error, Debug)]
#[error(transparent)]
pub enum CreateTagError {
    #[error("{0}")]
    Duplicate(String),
    #[error(transparent)]
    Db(#[from] eyre::Error),
}

#[nutype(derive(Debug, PartialEq, Eq, Hash))]
pub struct TagId(Uuid);

pub static TAG_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^[a-z0-9][a-z0-9-]*$").unwrap());
pub const TAG_NAME_MAX_LEN: usize = 32;
#[nutype(
    new_unchecked,
    validate(len_char_max = TAG_NAME_MAX_LEN, not_empty, regex = TAG_NAME_REGEX),
    derive(Debug, PartialEq, Eq, AsRef, Hash),
)]
pub struct TagName(String);

pub const TAG_ALIAS_MAX_LEN: usize = TAG_NAME_MAX_LEN;
#[nutype(
    new_unchecked,
    validate(len_char_max = TAG_ALIAS_MAX_LEN, not_empty),
    derive(Debug, PartialEq, Eq, AsRef, Hash),
)]
pub struct TagAlias(String);
