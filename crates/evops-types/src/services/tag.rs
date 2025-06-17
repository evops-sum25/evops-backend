use std::sync::LazyLock;

use nutype::nutype;
use regex::Regex;
use thiserror::Error;
use uuid::Uuid;

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

#[derive(Error, Debug)]
pub enum CreateTagError {
    #[error("{0}")]
    AlreadyExists(String),
    #[error(transparent)]
    Db(#[from] diesel::result::Error),
}

#[derive(Error, Debug)]
pub enum FindTagError {
    #[error("Tag with ID {0} was not found.")]
    NotFound(crate::TagId),
    #[error(transparent)]
    Db(#[from] diesel::result::Error),
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
