use std::collections::HashSet;
use std::sync::LazyLock;

use chrono::{DateTime, Utc};
use nutype::nutype;
use regex::Regex;
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

#[derive(Debug)]
pub struct CreateEventResponse {
    pub id: EventId,
    pub author: CreateUserResponse,
    pub images: HashSet<Url>,
    pub title: EventTitle,
    pub description: EventDescription,
    pub tags: HashSet<CreateTagResponse>,
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
    pub images: Option<HashSet<Url>>,
    pub title: EventTitle,
    pub description: EventDescription,
    pub tags: Option<HashSet<TagId>>,
    pub with_attendance: bool,
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
    pub aliases: Option<HashSet<TagAlias>>,
}

#[derive(Debug)]
pub struct CreateTagResponse {
    pub id: TagId,
    pub name: TagName,
    pub aliases: HashSet<TagAlias>,
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
