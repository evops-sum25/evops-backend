use thiserror::Error;

pub use self::services::event::{
    CreateEventError, EVENT_DESCRIPTION_MAX_LEN, EVENT_DESCRIPTION_MIN_LEN, EVENT_TITLE_MAX_LEN,
    EVENT_TITLE_MIN_LEN, Event, EventDescription, EventDescriptionError, EventId,
    EventServiceCreateRequest, EventServiceCreateResponse, EventServiceFindRequest,
    EventServiceFindResponse, EventServiceListRequest, EventServiceListResponse, EventTitle,
    EventTitleError, FindEventError, ListEventsError, NewEventForm,
};
pub use self::services::tag::{
    CreateTagError, FindTagError, ListTagsError, NewTagForm, TAG_ALIAS_MAX_LEN, TAG_ALIAS_MIN_LEN,
    TAG_NAME_MAX_LEN, TAG_NAME_MIN_LEN, TAG_NAME_REGEX, Tag, TagAlias, TagAliasError, TagId,
    TagName, TagNameError, TagServiceCreateRequest, TagServiceCreateResponse,
    TagServiceFindRequest, TagServiceFindResponse, TagServiceListRequest, TagServiceListResponse,
};
pub use self::services::user::{
    CreateUserError, FindUserError, ListUsersError, NewUserForm, USER_NAME_MAX_LEN,
    USER_NAME_MIN_LEN, User, UserId, UserName, UserNameError, UserServiceCreateRequest,
    UserServiceCreateResponse, UserServiceFindRequest, UserServiceFindResponse,
    UserServiceListRequest, UserServiceListResponse,
};

mod services;

#[derive(Debug, Error)]
#[error("{0}")]
pub enum ApiError {
    InvalidArgument(String),
    NotFound(String),
    AlreadyExists(String),
    #[error(transparent)]
    Db(#[from] diesel::result::Error),
    Other(String),
}
