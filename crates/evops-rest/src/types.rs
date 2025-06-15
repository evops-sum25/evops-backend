use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Deserialize, JsonSchema)]
pub struct EventServiceCreateRequest {
    author: self::UserId,
    images: Option<Vec<Url>>,
    title: self::EventTitle,
    description: self::EventDescription,
    tags: Option<Vec<self::TagId>>,
    with_attendance: bool,
}
impl TryFrom<self::EventServiceCreateRequest> for evops_types::CreateEventRequest {
    type Error = eyre::Error;

    fn try_from(value: self::EventServiceCreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            author: value.author.into(),
            images: value.images,
            title: value.title.into(),
            description: value.description.into(),
            tags: {
                value
                    .tags
                    .map(|it| it.into_iter().map(Into::into).collect())
            },
            with_attendance: value.with_attendance,
        })
    }
}

#[derive(Serialize, JsonSchema)]
pub struct EventServiceCreateResponse {
    author: self::UserServiceCreateResponse,
    images: Vec<Url>,
    title: self::EventTitle,
    description: self::EventDescription,
    tags: Vec<self::TagServiceCreateResponse>,
    with_attendance: bool,
}
impl From<evops_types::CreateEventResponse> for self::EventServiceCreateResponse {
    fn from(value: evops_types::CreateEventResponse) -> Self {
        Self {
            author: value.author.into(),
            images: value.images,
            title: value.title.into(),
            description: value.description.into(),
            tags: value.tags.into_iter().map(Into::into).collect(),
            with_attendance: value.with_attendance,
        }
    }
}

#[derive(Deserialize, JsonSchema)]
pub struct TagServiceCreateRequest {
    name: self::TagName,
    aliases: Option<Vec<self::TagAlias>>,
}
impl TryFrom<self::TagServiceCreateRequest> for evops_types::CreateTagRequest {
    type Error = eyre::Error;

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

#[derive(Serialize, JsonSchema)]
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

#[derive(Deserialize, JsonSchema)]
pub struct UserServiceCreateRequest {
    name: self::UserName,
    url: Option<Url>,
}
impl TryFrom<self::UserServiceCreateRequest> for evops_types::CreateUserRequest {
    type Error = eyre::Error;

    fn try_from(value: self::UserServiceCreateRequest) -> Result<Self, Self::Error> {
        Ok(evops_types::CreateUserRequest {
            name: value.name.try_into()?,
            profile_picture_url: value.url,
        })
    }
}

#[derive(Serialize, JsonSchema)]
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

#[derive(Serialize, Deserialize, JsonSchema)]
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

#[derive(Serialize, Deserialize, JsonSchema)]
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

#[derive(Serialize, Deserialize, JsonSchema)]
struct UserName(#[schemars(length(max = evops_types::USER_NAME_MAX_LEN))] String);
impl TryFrom<self::UserName> for evops_types::UserName {
    type Error = evops_types::UserNameError;

    fn try_from(value: self::UserName) -> Result<Self, Self::Error> {
        Ok(Self::try_new(value.0)?)
    }
}
impl From<evops_types::UserName> for self::UserName {
    fn from(value: evops_types::UserName) -> Self {
        Self(value.into_inner())
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct UserId(Uuid);
impl From<self::UserId> for evops_types::UserId {
    fn from(value: self::UserId) -> Self {
        evops_types::UserId::new(value.0)
    }
}
impl From<evops_types::UserId> for self::UserId {
    fn from(value: evops_types::UserId) -> Self {
        Self(value.into_inner())
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
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

#[derive(Serialize, Deserialize, JsonSchema)]
struct TagName(String);
impl TryFrom<self::TagName> for evops_types::TagName {
    type Error = evops_types::TagNameError;

    fn try_from(value: self::TagName) -> Result<Self, Self::Error> {
        Ok(Self::try_new(value.0)?)
    }
}
impl From<evops_types::TagName> for self::TagName {
    fn from(value: evops_types::TagName) -> Self {
        Self(value.into_inner())
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct TagAlias(String);
impl TryFrom<self::TagAlias> for evops_types::TagAlias {
    type Error = evops_types::TagAliasError;

    fn try_from(value: self::TagAlias) -> Result<Self, Self::Error> {
        Ok(Self::try_new(value.0)?)
    }
}
impl From<evops_types::TagAlias> for self::TagAlias {
    fn from(value: evops_types::TagAlias) -> Self {
        Self(value.into_inner())
    }
}
