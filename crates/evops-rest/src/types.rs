use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

mod conversions;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EventServiceCreateRequest {
    form: crate::types::NewEventForm,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct EventServiceCreateResponse {
    event: crate::types::Event,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TagServiceCreateRequest {
    form: crate::types::NewTagForm,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct TagServiceCreateResponse {
    tag: crate::types::Tag,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserServiceCreateRequest {
    form: crate::types::NewUserForm,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct UserServiceCreateResponse {
    user: crate::types::User,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct NewEventForm {
    author_id: crate::types::UserId,
    image_urls: Option<Vec<Url>>,
    title: crate::types::EventTitle,
    description: crate::types::EventDescription,
    tag_ids: Option<Vec<crate::types::TagId>>,
    with_attendance: bool,
}

#[derive(Debug, Serialize, JsonSchema)]
struct Event {
    id: crate::types::EventId,
    author: crate::types::User,
    image_urls: Vec<Url>,
    title: crate::types::EventTitle,
    description: crate::types::EventDescription,
    tags: Vec<crate::types::Tag>,
    with_attendance: bool,
    created_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct NewTagForm {
    name: crate::types::TagName,
    aliases: Option<Vec<crate::types::TagAlias>>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct Tag {
    id: TagId,
    name: crate::types::TagName,
    aliases: Vec<crate::types::TagAlias>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct NewUserForm {
    name: crate::types::UserName,
    profile_picture_url: Option<Url>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct User {
    id: crate::types::UserId,
    name: crate::types::UserName,
    profile_picture_url: Option<Url>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct EventId(Uuid);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct EventTitle(
    #[schemars(
        length(min = evops_types::EVENT_TITLE_MIN_LEN, max = evops_types::EVENT_TITLE_MAX_LEN),
    )]
    String,
);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct EventDescription(
    #[schemars(
        length(
            min = evops_types::EVENT_DESCRIPTION_MIN_LEN,
            max = evops_types::EVENT_DESCRIPTION_MAX_LEN,
        ),
    )]
    String,
);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct UserName(
    #[schemars(length(min = evops_types::USER_NAME_MIN_LEN, max = evops_types::USER_NAME_MAX_LEN))]
    String,
);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct UserId(Uuid);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct TagId(Uuid);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct TagName(
    #[schemars(
        length(min = evops_types::TAG_NAME_MIN_LEN, max = evops_types::TAG_NAME_MAX_LEN),
        regex(pattern = evops_types::TAG_NAME_REGEX),
        example = &"tag-like-on-github",
    )]
    String,
);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct TagAlias(
    #[schemars(
        length(min = evops_types::TAG_ALIAS_MIN_LEN, max = evops_types::TAG_ALIAS_MAX_LEN),
        example = &"alias-for-better-search-ux",
    )]
    String,
);
