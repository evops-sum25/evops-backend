use std::sync::LazyLock;

use nutype::nutype;
use regex::Regex;
use url::Url;
use uuid::Uuid;

pub struct CreateUserRequest {
    pub name: UserName,
    pub profile_picture_url: Option<Url>,
}

pub struct CreateUserResponse {
    pub id: UserId,
    pub name: UserName,
    pub profile_picture_url: Option<Url>,
}

pub struct CreateEventResponse {
    pub author: CreateUserResponse,
    pub images: Vec<Url>,
    pub title: EventTitle,
    pub description: EventDescription,
    pub tags: Vec<CreateTagResponse>,
    pub with_attendance: bool,
}

#[nutype(new_unchecked)]
pub struct EventTitle(String);

#[nutype(new_unchecked)]
pub struct EventDescription(String);

pub struct CreateEventRequest {
    pub author: UserId,
    pub images: Option<Vec<Url>>,
    pub title: EventTitle,
    pub description: EventDescription,
    pub tags: Option<Vec<TagId>>,
    pub with_attendance: bool,
}

#[nutype]
pub struct UserId(Uuid);

pub const USER_NAME_MAX_LEN: usize = 64;
#[nutype(new_unchecked, validate(len_char_max = USER_NAME_MAX_LEN))]
pub struct UserName(String);

pub struct CreateTagRequest {
    pub name: TagName,
    pub aliases: Option<Vec<TagAlias>>,
}

pub struct CreateTagResponse {
    pub id: TagId,
    pub name: TagName,
    pub aliases: Vec<TagAlias>,
}

#[nutype]
pub struct TagId(Uuid);

pub static TAG_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^[a-z0-9][a-z0-9-]*$").unwrap());
pub const TAG_NAME_MAX_LEN: usize = 32;
#[nutype(
    new_unchecked,
    validate(len_char_max = TAG_NAME_MAX_LEN, not_empty, regex = TAG_NAME_REGEX),
)]
pub struct TagName(String);

pub const TAG_ALIAS_MAX_LEN: usize = TAG_NAME_MAX_LEN;
#[nutype(new_unchecked, validate(len_char_max = TAG_ALIAS_MAX_LEN))]
pub struct TagAlias(String);
