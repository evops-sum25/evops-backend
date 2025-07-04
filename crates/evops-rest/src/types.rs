use aide_axum_typed_multipart::FieldData;
use axum::body::Bytes;
use axum_typed_multipart::TryFromMultipart;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod conversions;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EventServicePushImageRequestPath {
    /// UUID of the event to add an image to.
    pub id: crate::types::EventId,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EventServiceFindRequest {
    /// UUID of the event to retrieve.
    pub id: crate::types::EventId,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EventServiceFindImageRequest {
    /// UUID of the event image to retrieve.
    pub id: crate::types::EventImageId,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct EventServiceFindResponse {
    /// Retrieved event object.
    pub event: crate::types::Event,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EventServiceListRequest {
    /// UUID of last listed event.
    pub last_id: Option<crate::types::EventId>,
    /// Size of one batch of events.
    pub limit: Option<crate::types::PgLimit>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct EventServiceListResponse {
    /// List of all events.
    pub events: Vec<crate::types::Event>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EventServiceCreateRequest {
    /// Data for creating a new event.
    pub form: crate::types::NewEventForm,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct EventServiceCreateResponse {
    /// ID of the created event.
    pub event_id: crate::types::EventId,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TagServiceFindRequest {
    /// UUID of the tag to retrieve.
    pub id: crate::types::TagId,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct TagServiceFindResponse {
    /// Retrieved tag object.
    pub tag: crate::types::Tag,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TagServiceListRequest {
    /// UUID of last listed event.
    pub last_id: Option<crate::types::TagId>,
    /// Size of one batch of events.
    pub limit: Option<crate::types::PgLimit>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct TagServiceListResponse {
    /// List of all tags.
    pub tags: Vec<crate::types::Tag>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TagServiceCreateRequest {
    /// Data for creating a new tag.
    pub form: crate::types::NewTagForm,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct TagServiceCreateResponse {
    /// ID of the created tag.
    pub tag_id: crate::types::TagId,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserServiceFindRequest {
    /// UUID of the user to retrieve.
    pub id: crate::types::UserId,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct UserServiceFindResponse {
    /// Retrieved user object.
    pub user: crate::types::User,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserServiceListRequest;

#[derive(Debug, Serialize, JsonSchema)]
pub struct UserServiceListResponse {
    /// List of all users.
    pub users: Vec<crate::types::User>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserServiceCreateRequest {
    /// Data for creating a new user.
    pub form: crate::types::NewUserForm,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct UserServiceCreateResponse {
    /// ID of the created user.
    pub user_id: crate::types::UserId,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct EventTagIds(
    #[schemars(length(max = evops_models::EVENT_MAX_TAGS))] Vec<crate::types::TagId>,
);

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NewEventForm {
    /// UUID of the creating user.
    author_id: crate::types::UserId,
    /// Event title.
    title: crate::types::EventTitle,
    /// Detailed description.
    description: crate::types::EventDescription,
    /// UUIDs of associated tags.
    tag_ids: Option<EventTagIds>,
    /// Whether to enable attendance tracking.
    with_attendance: bool,
}

#[derive(Debug, Serialize, JsonSchema)]
struct EventTags(#[schemars(length(max = evops_models::EVENT_MAX_TAGS))] Vec<crate::types::Tag>);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EventImageId(Uuid);

#[derive(Debug, Serialize, JsonSchema)]
struct EventImageIds(
    #[schemars(length(max = evops_models::EVENT_MAX_IMAGES))] Vec<crate::types::EventImageId>,
);

#[derive(Debug, TryFromMultipart, JsonSchema)]
pub struct EventServicePushImageRequestMultipart {
    pub image: FieldData<Bytes>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct EventServicePushImageResponse {
    pub image_id: crate::types::EventImageId,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct Event {
    /// Event UUID.
    id: crate::types::EventId,
    /// User who created the event.
    author: crate::types::User,
    /// UUIDs of event images.
    image_ids: crate::types::EventImageIds,
    /// Event title.
    title: crate::types::EventTitle,
    /// Detailed description.
    description: crate::types::EventDescription,
    /// Associated tags.
    tags: crate::types::EventTags,
    /// Whether attendance tracking is enabled.
    with_attendance: bool,
    /// Creation timestamp.
    created_at: DateTime<Utc>,
    /// Last modification timestamp.
    modified_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
struct TagAliases(
    #[schemars(length(max = evops_models::TAG_MAX_ALIASES))] Vec<crate::types::TagAlias>,
);

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NewTagForm {
    /// Unique name.
    name: crate::types::TagName,
    /// Alternative names.
    aliases: Option<crate::types::TagAliases>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct Tag {
    /// Tag UUID.
    id: TagId,
    /// Unique name (e.g., `"Music"`, `"Tech"`).
    name: crate::types::TagName,
    /// Alternative names (e.g., `["Concert", "Gig"]` for `"Music"`).
    aliases: crate::types::TagAliases,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NewUserForm {
    /// Display name.
    name: crate::types::UserName,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct User {
    /// User UUID.
    id: crate::types::UserId,
    /// Display name.
    name: crate::types::UserName,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EventId(Uuid);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct EventTitle(
    #[schemars(
        length(min = evops_models::EVENT_TITLE_MIN_LEN, max = evops_models::EVENT_TITLE_MAX_LEN),
    )]
    String,
);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct EventDescription(
    #[schemars(
        length(
            min = evops_models::EVENT_DESCRIPTION_MIN_LEN,
            max = evops_models::EVENT_DESCRIPTION_MAX_LEN,
        ),
    )]
    String,
);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct UserName(
    #[schemars(length(min = evops_models::USER_NAME_MIN_LEN, max = evops_models::USER_NAME_MAX_LEN))]
     String,
);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UserId(Uuid);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TagId(Uuid);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct TagName(
    #[schemars(
        length(min = evops_models::TAG_NAME_MIN_LEN, max = evops_models::TAG_NAME_MAX_LEN),
        regex(pattern = evops_models::TAG_NAME_REGEX),
        example = &"tag-like-on-github",
    )]
    String,
);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct TagAlias(
    #[schemars(
        length(min = evops_models::TAG_ALIAS_MIN_LEN, max = evops_models::TAG_ALIAS_MAX_LEN),
        example = &"alias-for-better-search-ux",
    )]
    String,
);

/// Non-negative integer for pagination limits.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct PgLimit(#[schemars(range(min = 0))] i64);
