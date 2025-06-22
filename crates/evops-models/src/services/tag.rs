use std::sync::LazyLock;

use nutype::nutype;
use regex::Regex;
use uuid::Uuid;

#[derive(Debug)]
pub struct TagServiceFindRequest {
    pub id: crate::TagId,
}

#[derive(Debug)]
pub struct TagServiceFindResponse {
    pub tag: crate::Tag,
}

#[derive(Debug)]
pub struct TagServiceListRequest;

#[derive(Debug)]
pub struct TagServiceListResponse {
    pub tags: Vec<crate::Tag>,
}

#[derive(Debug)]
pub struct TagServiceCreateRequest {
    pub form: crate::NewTagForm,
}

#[derive(Debug)]
pub struct TagServiceCreateResponse {
    pub tag: crate::Tag,
}

#[derive(Debug)]
pub struct NewTagForm {
    pub name: crate::TagName,
    pub aliases: Option<Vec<crate::TagAlias>>,
}

#[derive(Debug)]
pub struct Tag {
    pub id: crate::TagId,
    pub name: crate::TagName,
    pub aliases: Vec<crate::TagAlias>,
}

#[nutype(derive(Debug, Clone, Copy, Debug, PartialEq, Eq, Hash, Display))]
pub struct TagId(Uuid);

pub static TAG_NAME_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^[a-z0-9][a-z0-9-]*$").unwrap());
pub const TAG_NAME_MIN_LEN: usize = 1;
pub const TAG_NAME_MAX_LEN: usize = 32;
#[nutype(
    new_unchecked,
    validate(len_char_max = TAG_NAME_MAX_LEN, not_empty, regex = TAG_NAME_REGEX),
    derive(Debug, PartialEq, Eq, AsRef, Hash, Display),
)]
pub struct TagName(String);

pub const TAG_ALIAS_MIN_LEN: usize = 1;
pub const TAG_ALIAS_MAX_LEN: usize = TAG_NAME_MAX_LEN;
#[nutype(
    new_unchecked,
    validate(len_char_max = TAG_ALIAS_MAX_LEN, not_empty),
    derive(Debug, PartialEq, Eq, AsRef, Hash, Display),
)]
pub struct TagAlias(String);
