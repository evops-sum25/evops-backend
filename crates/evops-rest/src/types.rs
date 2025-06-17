use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EventServiceCreateRequest {
    author_id: self::UserId,
    image_urls: Option<Vec<Url>>,
    title: self::EventTitle,
    description: self::EventDescription,
    tag_ids: Option<Vec<self::TagId>>,
    with_attendance: bool,
}
impl TryFrom<self::EventServiceCreateRequest> for evops_types::CreateEventRequest {
    type Error = crate::error::Error;

    fn try_from(value: self::EventServiceCreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            author_id: value.author_id.into(),
            image_urls: value.image_urls,
            title: value.title.into(),
            description: value.description.into(),
            tag_ids: {
                value
                    .tag_ids
                    .map(|t| t.into_iter().map(Into::into).collect())
            },
            with_attendance: value.with_attendance,
        })
    }
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct EventServiceCreateResponse {
    id: self::EventId,
    author: self::UserServiceCreateResponse,
    image_urls: Vec<Url>,
    title: self::EventTitle,
    description: self::EventDescription,
    tags: Vec<self::TagServiceCreateResponse>,
    with_attendance: bool,
    created_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
}
impl From<evops_types::CreateEventResponse> for self::EventServiceCreateResponse {
    fn from(value: evops_types::CreateEventResponse) -> Self {
        Self {
            id: value.id.into(),
            author: value.author.into(),
            image_urls: value.image_urls,
            title: value.title.into(),
            description: value.description.into(),
            tags: value.tags.into_iter().map(Into::into).collect(),
            with_attendance: value.with_attendance,
            created_at: value.created_at,
            modified_at: value.modified_at,
        }
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TagServiceCreateRequest {
    name: self::TagName,
    aliases: Option<Vec<self::TagAlias>>,
}
impl TryFrom<self::TagServiceCreateRequest> for evops_types::CreateTagRequest {
    type Error = crate::error::UnprocessableEntityError;

    fn try_from(value: self::TagServiceCreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.try_into()?,
            aliases: match value.aliases {
                Some(it) => Some({
                    it.into_iter()
                        .map(TryInto::try_into)
                        .collect::<Result<_, _>>()?
                }),
                None => None,
            },
        })
    }
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct TagServiceCreateResponse {
    id: TagId,
    name: self::TagName,
    aliases: Vec<self::TagAlias>,
}
impl From<evops_types::CreateTagResponse> for TagServiceCreateResponse {
    fn from(value: evops_types::CreateTagResponse) -> Self {
        Self {
            id: value.id.into(),
            name: value.name.into(),
            aliases: value.aliases.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserServiceCreateRequest {
    name: self::UserName,
    profile_picture_url: Option<Url>,
}
impl TryFrom<self::UserServiceCreateRequest> for evops_types::CreateUserRequest {
    type Error = crate::error::UnprocessableEntityError;

    fn try_from(value: self::UserServiceCreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.try_into()?,
            profile_picture_url: value.profile_picture_url,
        })
    }
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct UserServiceCreateResponse {
    id: self::UserId,
    name: self::UserName,
    profile_picture_url: Option<Url>,
}
impl From<evops_types::CreateUserResponse> for self::UserServiceCreateResponse {
    fn from(value: evops_types::CreateUserResponse) -> Self {
        Self {
            id: value.id.into(),
            name: value.name.into(),
            profile_picture_url: value.profile_picture_url,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct EventId(Uuid);
impl From<self::EventId> for evops_types::EventId {
    fn from(value: self::EventId) -> Self {
        Self::new(value.0)
    }
}
impl From<evops_types::EventId> for self::EventId {
    fn from(value: evops_types::EventId) -> Self {
        Self(value.into_inner())
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct EventTitle(String);
impl From<self::EventTitle> for evops_types::EventTitle {
    fn from(value: self::EventTitle) -> Self {
        Self::new(value.0)
    }
}
impl From<evops_types::EventTitle> for self::EventTitle {
    fn from(value: evops_types::EventTitle) -> Self {
        Self(value.into_inner())
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct EventDescription(String);
impl From<self::EventDescription> for evops_types::EventDescription {
    fn from(value: self::EventDescription) -> Self {
        Self::new(value.0)
    }
}
impl From<evops_types::EventDescription> for self::EventDescription {
    fn from(value: evops_types::EventDescription) -> Self {
        Self(value.into_inner())
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct UserName(#[schemars(length(min = 1, max = evops_types::USER_NAME_MAX_LEN))] String);
impl TryFrom<self::UserName> for evops_types::UserName {
    type Error = crate::error::UnprocessableEntityError;

    fn try_from(value: self::UserName) -> Result<Self, Self::Error> {
        Ok(Self::try_new(value.0)?)
    }
}
impl From<evops_types::UserName> for self::UserName {
    fn from(value: evops_types::UserName) -> Self {
        Self(value.into_inner())
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct UserId(Uuid);
impl From<self::UserId> for evops_types::UserId {
    fn from(value: self::UserId) -> Self {
        Self::new(value.0)
    }
}
impl From<evops_types::UserId> for self::UserId {
    fn from(value: evops_types::UserId) -> Self {
        Self(value.into_inner())
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct TagId(Uuid);
impl From<self::TagId> for evops_types::TagId {
    fn from(value: self::TagId) -> Self {
        Self::new(value.0)
    }
}
impl From<evops_types::TagId> for self::TagId {
    fn from(value: evops_types::TagId) -> Self {
        Self(value.into_inner())
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct TagName(
    #[schemars(
        length(min = 1, max = evops_types::TAG_NAME_MAX_LEN),
        regex(pattern = evops_types::TAG_NAME_REGEX),
        example = &"tag-like-on-github",
    )]
    String,
);
impl TryFrom<self::TagName> for evops_types::TagName {
    type Error = crate::error::UnprocessableEntityError;

    fn try_from(value: self::TagName) -> Result<Self, Self::Error> {
        Ok(Self::try_new(value.0)?)
    }
}
impl From<evops_types::TagName> for self::TagName {
    fn from(value: evops_types::TagName) -> Self {
        Self(value.into_inner())
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct TagAlias(
    #[schemars(
        length(min = 1, max = evops_types::TAG_ALIAS_MAX_LEN),
        example = &"alias-for-better-search-ux",
    )]
    String,
);
impl TryFrom<self::TagAlias> for evops_types::TagAlias {
    type Error = crate::error::UnprocessableEntityError;

    fn try_from(value: self::TagAlias) -> Result<Self, Self::Error> {
        Ok(Self::try_new(value.0)?)
    }
}
impl From<evops_types::TagAlias> for self::TagAlias {
    fn from(value: evops_types::TagAlias) -> Self {
        Self(value.into_inner())
    }
}
