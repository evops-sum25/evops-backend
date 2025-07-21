use aide_axum_typed_multipart::FieldData;
use axum::body::Bytes;
use axum_typed_multipart::TryFromMultipart;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod impls;

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct TagServiceDeleteRequestPath {
    pub tag_id: TagId,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EventServiceReorderImagesRequest {
    pub image_ids: EventImageIds,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct EventServiceDeleteImageRequestPath {
    pub _image_id: EventImageId,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EventServiceUpdateRequest {
    /// Data for updating an event.
    pub form: UpdateEventForm,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateEventForm {
    /// Event title.
    title: Option<EventTitle>,
    /// Detailed description.
    description: Option<EventDescription>,
    /// UUIDs of associated tags.
    tag_ids: Option<EventTagIds>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct EventServiceUpdateRequestPath {
    /// UUID of the event to be updated.
    pub event_id: EventId,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct EventServiceDeleteRequestPath {
    /// UUID of the event to be deleted.
    pub event_id: EventId,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LanguageServiceAddRequest {
    pub form: NewLanguageForm,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NewLanguageForm {
    name: LanguageName,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct LanguageServiceAddResponse {
    pub language_id: LanguageId,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LanguageId(Uuid);

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LanguageName(
    #[schemars(length(
        min = evops_models::LanguageName::LEN_CHAR_MIN,
        max = evops_models::LanguageName::LEN_CHAR_MAX,
    ))]
    String,
);

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct EventServicePushImageRequestPath {
    /// UUID of the event to add an image to.
    pub event_id: EventId,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct EventServiceReorderImageRequestPath {
    /// UUID of the event to reorder images for.
    pub event_id: EventId,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct EventServiceFindRequestPath {
    /// UUID of the event to retrieve.
    pub event_id: EventId,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct EventServiceFindImageRequestPath {
    /// UUID of the event image to retrieve.
    pub image_id: EventImageId,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct EventServiceFindResponse {
    /// Retrieved event object.
    pub event: Event,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct EventServiceListRequestQuery {
    /// UUID of last listed event.
    pub last_id: Option<EventId>,
    /// Size of one batch of events.
    pub limit: Option<PgLimit>,
    /// Tag ids of events to be listed.
    #[serde(default)]
    #[serde(rename = "tag-id")]
    pub tag_ids: Vec<TagId>,
    /// Search string to match against event titles and descriptions (case-insensitive).
    pub search: Option<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct EventServiceListResponse {
    /// List of all events.
    pub events: Vec<Event>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EventServiceCreateRequest {
    /// Data for creating a new event.
    pub form: NewEventForm,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct EventServiceCreateResponse {
    /// ID of the created event.
    pub event_id: EventId,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct TagServiceFindRequestPath {
    /// UUID of the tag to retrieve.
    pub tag_id: TagId,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct TagServiceFindResponse {
    /// Retrieved tag object.
    pub tag: Tag,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct TagServiceListRequestQuery {
    /// UUID of last listed event.
    pub last_id: Option<TagId>,
    /// Size of one batch of events.
    pub limit: Option<PgLimit>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct TagServiceListResponse {
    /// List of all tags.
    pub tags: Vec<Tag>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TagServiceCreateRequest {
    /// Data for creating a new tag.
    pub form: NewTagForm,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct TagServiceCreateResponse {
    /// ID of the created tag.
    pub tag_id: TagId,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct TagServiceSuggestRequestQuery {
    /// Description to predict tags for.
    pub description: EventDescription,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct TagServiceSuggestResponse {
    /// A list of predicted tags for a description.
    pub tags: Vec<Tag>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UserServiceListRequest;

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct EventTagIds(
    #[schemars(length(max = evops_models::EventTagIds::ITEMS_MAX))] pub Vec<TagId>,
);

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NewEventForm {
    /// Event title.
    title: EventTitle,
    /// Detailed description.
    description: EventDescription,
    /// UUIDs of associated tags.
    tag_ids: Option<EventTagIds>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct EventTags(#[schemars(length(max = evops_models::EventTags::ITEMS_MAX))] Vec<Tag>);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EventImageId(Uuid);

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct EventImageIds(
    #[schemars(length(max = evops_models::EventImageIds::ITEMS_MAX))] Vec<EventImageId>,
);

#[derive(Debug, TryFromMultipart, JsonSchema)]
pub struct EventServicePushImageRequestMultipart {
    pub image: FieldData<Bytes>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct EventServicePushImageResponse {
    pub image_id: EventImageId,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct Event {
    /// Event UUID.
    id: EventId,
    /// User who created the event.
    author: User,
    /// UUIDs of event images.
    image_ids: EventImageIds,
    /// Event title.
    title: EventTitle,
    /// Detailed description.
    description: EventDescription,
    /// Associated tags.
    tags: EventTags,
    /// Creation timestamp.
    created_at: DateTime<Utc>,
    /// Last modification timestamp.
    modified_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
struct TagAliases(#[schemars(length(max = evops_models::TagAliases::ITEMS_MAX))] Vec<TagAlias>);

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NewTagForm {
    /// Unique name.
    name: TagName,
    /// Alternative names.
    aliases: Option<TagAliases>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct Tag {
    /// Tag UUID.
    id: TagId,
    /// Unique name (e.g., `"Music"`, `"Tech"`).
    name: TagName,
    /// Alternative names (e.g., `["Concert", "Gig"]` for `"Music"`).
    aliases: TagAliases,
}

#[derive(Deserialize, JsonSchema)]
pub struct NewUserForm {
    /// Case-insensitively unique login.
    login: UserLogin,
    /// Display name.
    display_name: Option<UserDisplayName>,
    password: UserPassword,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UserLogin(
    #[schemars(
        length(
            min = evops_models::UserLogin::LEN_CHAR_MIN,
            max = evops_models::UserLogin::LEN_CHAR_MAX,
        ),
        regex(pattern = evops_models::UserLogin::REGEX),
    )]
    String,
);

#[derive(Debug, Serialize, JsonSchema)]
pub struct User {
    /// User UUID.
    id: UserId,
    /// Case-insensitively unique login.
    login: UserLogin,
    /// Display name.
    display_name: UserDisplayName,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct JsonWebToken(#[schemars(regex(pattern = r"^[\w-]+\.[\w-]+\.[\w-]+$"))] String);

#[derive(Serialize, JsonSchema)]
pub struct AuthTokens {
    access: JsonWebToken,
    refresh: JsonWebToken,
}

#[derive(Serialize, JsonSchema)]
pub struct AuthServiceSignUpResponse {
    pub tokens: AuthTokens,
}

#[derive(Serialize, JsonSchema)]
pub struct AuthServiceGetMyInfoResponse {
    pub user: User,
}

#[derive(Serialize, JsonSchema)]
pub struct AuthServiceLogInResponse {
    pub tokens: AuthTokens,
}

#[derive(Serialize, JsonSchema)]
pub struct AuthServiceRefreshSessionResponse {
    pub tokens: AuthTokens,
}

#[derive(Deserialize, JsonSchema)]
pub struct AuthServiceRefreshSessionRequest {
    pub refresh_token: JsonWebToken,
}

#[derive(Deserialize, JsonSchema)]
pub struct AuthServiceLogInRequest {
    pub credentials: UserCredentials,
}

#[derive(Deserialize, JsonSchema)]
pub struct AuthServiceSignUpRequest {
    pub form: NewUserForm,
}

#[derive(Deserialize, JsonSchema)]
pub struct UserCredentials {
    pub login: UserLogin,
    pub password: UserPassword,
}

#[derive(Deserialize, JsonSchema)]
pub struct UserPassword(
    #[schemars(
        length(
            min = evops_models::UserPassword::LEN_CHAR_MIN,
            max = evops_models::UserPassword::LEN_CHAR_MAX,
        ),
        regex(pattern = evops_models::UserPassword::REGEX),
    )]
    String,
);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EventId(Uuid);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EventTitle(
    #[schemars(length(
        min = evops_models::EventTitle::LEN_CHAR_MIN,
        max = evops_models::EventTitle::LEN_CHAR_MAX,
    ))]
    String,
);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EventDescription(
    #[schemars(length(
        min = evops_models::EventDescription::LEN_CHAR_MIN,
        max = evops_models::EventDescription::LEN_CHAR_MAX,
    ))]
    String,
);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct UserDisplayName(
    #[schemars(length(
        min = evops_models::UserDisplayName::LEN_CHAR_MIN,
        max = evops_models::UserDisplayName::LEN_CHAR_MAX,
    ))]
    String,
);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UserId(Uuid);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TagId(Uuid);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct TagName(
    #[schemars(
        length(
            min = evops_models::TagName::LEN_CHAR_MIN,
            max = evops_models::TagName::LEN_CHAR_MAX,
        ),
        regex(pattern = evops_models::TagName::REGEX),
        example = &"format-like-topics-on-github",
    )]
    String,
);

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct TagAlias(
    #[schemars(
        length(
            min = evops_models::TagAlias::LEN_CHAR_MIN,
            max = evops_models::TagAlias::LEN_CHAR_MAX,
        ),
        example = &"alias-for-better-search-ux",
    )]
    String,
);

/// Non-negative integer for pagination limits.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct PgLimit(#[schemars(range(min = 0))] i64);
