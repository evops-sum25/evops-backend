use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

mod conversions;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EventServiceFindRequest {
    /// UUID of the event to retrieve.
    id: crate::types::EventId,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct EventServiceFindResponse {
    /// Retrieved event object.
    event: crate::types::Event,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EventServiceListRequest {
    /// UUID of last listed event.
    last_id: Option<crate::types::EventId>,
    /// Size of one batch of events.
    limit: Option<crate::types::PgLimit>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct EventServiceListResponse {
    /// List of all events.
    events: Vec<crate::types::Event>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EventServiceCreateRequest {
    /// Data for creating a new event.
    form: crate::types::NewEventForm,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct EventServiceCreateResponse {
    /// Created event object.
    event: crate::types::Event,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TagServiceFindRequest {
    /// UUID of the tag to retrieve.
    id: crate::types::TagId,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct TagServiceFindResponse {
    /// Retrieved tag object.
    tag: crate::types::Tag,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TagServiceListRequest {
    /// UUID of last listed event.
    last_id: Option<crate::types::TagId>,
    /// Size of one batch of events.
    limit: Option<crate::types::PgLimit>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct TagServiceListResponse {
    /// List of all tags.
    tags: Vec<crate::types::Tag>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TagServiceCreateRequest {
    /// Data for creating a new tag.
    form: crate::types::NewTagForm,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct TagServiceCreateResponse {
    /// Created tag object.
    tag: crate::types::Tag,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserServiceFindRequest {
    /// UUID of the user to retrieve.
    id: crate::types::UserId,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct UserServiceFindResponse {
    /// Retrieved user object.
    user: crate::types::User,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserServiceListRequest;

#[derive(Debug, Serialize, JsonSchema)]
pub struct UserServiceListResponse {
    /// List of all users.
    users: Vec<crate::types::User>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserServiceCreateRequest {
    /// Data for creating a new user.
    form: crate::types::NewUserForm,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct UserServiceCreateResponse {
    /// Created user object.
    user: crate::types::User,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct NewEventForm {
    /// UUID of the creating user.
    author_id: crate::types::UserId,
    /// URLs of event images.
    image_urls: Option<Vec<Url>>,
    /// Event title.
    title: crate::types::EventTitle,
    /// Detailed description.
    description: crate::types::EventDescription,
    /// UUIDs of associated tags.
    tag_ids: Option<Vec<crate::types::TagId>>,
    /// Whether to enable attendance tracking.
    with_attendance: bool,
}

#[derive(Debug, Serialize, JsonSchema)]
struct Event {
    /// Event UUID.
    id: crate::types::EventId,
    /// User who created the event.
    author: crate::types::User,
    /// URLs of event images.
    image_urls: Vec<Url>,
    /// Event title.
    title: crate::types::EventTitle,
    /// Detailed description.
    description: crate::types::EventDescription,
    /// Associated tags.
    tags: Vec<crate::types::Tag>,
    /// Whether attendance tracking is enabled.
    with_attendance: bool,
    /// Creation timestamp.
    created_at: DateTime<Utc>,
    /// Last modification timestamp.
    modified_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct NewTagForm {
    /// Unique name.
    name: crate::types::TagName,
    /// Alternative names.
    aliases: Option<Vec<crate::types::TagAlias>>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct Tag {
    /// Tag UUID.
    id: TagId,
    /// Unique name (e.g., `"Music"`, `"Tech"`).
    name: crate::types::TagName,
    /// Alternative names (e.g., `["Concert", "Gig"]` for `"Music"`).
    aliases: Vec<crate::types::TagAlias>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct NewUserForm {
    /// Display name.
    name: crate::types::UserName,
}

#[derive(Debug, Serialize, JsonSchema)]
struct User {
    /// User UUID.
    id: crate::types::UserId,
    /// Display name.
    name: crate::types::UserName,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct EventId(Uuid);

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
struct UserId(Uuid);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct TagId(Uuid);

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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct PgLimit(
    #[schemars(
        range(min = 0),
        description = "Non-negative integer for database limits"
    )]
    i64,
);
