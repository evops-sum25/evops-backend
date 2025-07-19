use evops_models::ApiError;

use crate::types::{
    AuthTokens, Event, EventDescription, EventId, EventImageId, EventImageIds, EventTagIds,
    EventTags, EventTitle, JsonWebToken, LanguageId, LanguageName, NewEventForm, NewLanguageForm,
    NewTagForm, NewUserForm, PgLimit, Tag, TagAlias, TagAliases, TagId, TagName, UpdateEventForm,
    User, UserDisplayName, UserId, UserLogin, UserPassword,
};

impl TryFrom<NewLanguageForm> for evops_models::NewLanguageForm {
    type Error = ApiError;

    fn try_from(value: NewLanguageForm) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.try_into()?,
        })
    }
}

impl From<JsonWebToken> for evops_models::JsonWebToken {
    fn from(value: JsonWebToken) -> Self {
        Self::new(value.0)
    }
}

impl From<evops_models::AuthTokens> for AuthTokens {
    fn from(value: evops_models::AuthTokens) -> Self {
        Self {
            access: value.access.into(),
            refresh: value.refresh.into(),
        }
    }
}

impl From<evops_models::JsonWebToken> for JsonWebToken {
    fn from(value: evops_models::JsonWebToken) -> Self {
        Self(value.into_inner())
    }
}

impl TryFrom<LanguageName> for evops_models::LanguageName {
    type Error = ApiError;

    fn try_from(value: LanguageName) -> Result<Self, Self::Error> {
        Self::try_new(value.0).map_err(|e| ApiError::InvalidArgument(e.to_string()))
    }
}

impl From<evops_models::LanguageId> for LanguageId {
    fn from(value: evops_models::LanguageId) -> Self {
        Self(value.into_inner())
    }
}

impl TryFrom<NewEventForm> for evops_models::NewEventForm {
    type Error = ApiError;

    fn try_from(value: NewEventForm) -> Result<Self, Self::Error> {
        Ok(Self {
            title: value.title.try_into()?,
            description: value.description.try_into()?,
            tag_ids: value.tag_ids.unwrap_or_default().try_into()?,
        })
    }
}

impl TryFrom<UpdateEventForm> for evops_models::UpdateEventForm {
    type Error = ApiError;

    fn try_from(value: UpdateEventForm) -> Result<Self, Self::Error> {
        Ok(Self {
            title: value.title.map(TryInto::try_into).transpose()?,
            description: value.description.map(TryInto::try_into).transpose()?,
            tag_ids: value.tag_ids.map(TryInto::try_into).transpose()?,
        })
    }
}

impl TryFrom<EventTagIds> for evops_models::EventTagIds {
    type Error = ApiError;

    fn try_from(value: EventTagIds) -> Result<Self, Self::Error> {
        Self::try_new(value.0.into_iter().map(Into::into).collect())
            .map_err(|e| ApiError::InvalidArgument(e.to_string()))
    }
}

impl From<EventImageId> for evops_models::EventImageId {
    fn from(value: EventImageId) -> Self {
        Self::new(value.0)
    }
}

impl From<EventImageIds> for evops_models::EventImageIds {
    fn from(value: EventImageIds) -> Self {
        // SAFETY:
        // 1. EventImageId::from is infallible (as shown above)
        // 2. Vec construction cannot fail
        // 3. new_unchecked is just an optimization of new() with same guarantees
        unsafe {
            Self::new_unchecked(
                value
                    .0
                    .into_iter()
                    .map(evops_models::EventImageId::from)
                    .collect(),
            )
        }
    }
}

impl From<evops_models::EventImageId> for EventImageId {
    fn from(value: evops_models::EventImageId) -> Self {
        Self(value.into_inner())
    }
}

impl From<evops_models::EventImageIds> for EventImageIds {
    fn from(value: evops_models::EventImageIds) -> Self {
        Self(value.into_inner().into_iter().map(Into::into).collect())
    }
}

impl From<evops_models::EventTags> for EventTags {
    fn from(value: evops_models::EventTags) -> Self {
        Self(value.into_inner().into_iter().map(Into::into).collect())
    }
}

impl From<evops_models::Event> for Event {
    fn from(value: evops_models::Event) -> Self {
        Self {
            id: value.id.into(),
            title: value.title.into(),
            description: value.description.into(),
            author: value.author.into(),
            image_ids: value.image_ids.into(),
            tags: value.tags.into(),
            created_at: value.created_at,
            modified_at: value.modified_at,
        }
    }
}

impl TryFrom<TagAliases> for evops_models::TagAliases {
    type Error = ApiError;

    fn try_from(value: TagAliases) -> Result<Self, Self::Error> {
        Self::try_new({
            value
                .0
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?
        })
        .map_err(|e| ApiError::InvalidArgument(e.to_string()))
    }
}

impl From<evops_models::TagAliases> for TagAliases {
    fn from(value: evops_models::TagAliases) -> Self {
        Self(value.into_inner().into_iter().map(Into::into).collect())
    }
}

impl TryFrom<NewTagForm> for evops_models::NewTagForm {
    type Error = ApiError;

    fn try_from(value: NewTagForm) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.try_into()?,
            aliases: value.aliases.unwrap_or_default().try_into()?,
        })
    }
}

impl From<evops_models::Tag> for Tag {
    fn from(value: evops_models::Tag) -> Self {
        Self {
            id: value.id.into(),
            name: value.name.into(),
            aliases: value.aliases.into(),
        }
    }
}

impl TryFrom<NewUserForm> for evops_models::NewUserForm {
    type Error = ApiError;

    fn try_from(value: NewUserForm) -> Result<Self, Self::Error> {
        let login: evops_models::UserLogin = value.login.try_into()?;
        Ok(Self {
            display_name: match value.display_name {
                Some(display_name) => display_name.try_into()?,
                None => unsafe {
                    evops_models::UserDisplayName::new_unchecked(login.as_ref().to_owned())
                },
            },
            login,
            password: value.password.try_into()?,
        })
    }
}

impl TryFrom<UserPassword> for evops_models::UserPassword {
    type Error = ApiError;

    fn try_from(value: UserPassword) -> Result<Self, Self::Error> {
        Self::try_new(value.0).map_err(|e| ApiError::InvalidArgument(e.to_string()))
    }
}

impl From<evops_models::User> for User {
    fn from(value: evops_models::User) -> Self {
        Self {
            id: value.id.into(),
            login: value.login.into(),
            display_name: value.display_name.into(),
        }
    }
}

impl TryFrom<UserLogin> for evops_models::UserLogin {
    type Error = ApiError;

    fn try_from(value: UserLogin) -> Result<Self, Self::Error> {
        Self::try_new(value.0).map_err(|e| ApiError::InvalidArgument(e.to_string()))
    }
}

impl From<evops_models::UserLogin> for UserLogin {
    fn from(value: evops_models::UserLogin) -> Self {
        Self(value.into_inner())
    }
}

impl From<evops_models::EventId> for EventId {
    fn from(value: evops_models::EventId) -> Self {
        Self(value.into_inner())
    }
}

impl From<EventId> for evops_models::EventId {
    fn from(value: EventId) -> Self {
        Self::new(value.0)
    }
}

impl From<evops_models::PgLimit> for PgLimit {
    fn from(value: evops_models::PgLimit) -> Self {
        Self(value.into_inner())
    }
}

impl TryFrom<PgLimit> for evops_models::PgLimit {
    type Error = ApiError;

    fn try_from(value: PgLimit) -> Result<Self, Self::Error> {
        Self::try_new(value.0).map_err(|e| ApiError::InvalidArgument(e.to_string()))
    }
}

impl TryFrom<EventTitle> for evops_models::EventTitle {
    type Error = ApiError;

    fn try_from(value: EventTitle) -> Result<Self, Self::Error> {
        Self::try_new(value.0).map_err(|e| ApiError::InvalidArgument(e.to_string()))
    }
}

impl From<evops_models::EventTitle> for EventTitle {
    fn from(value: evops_models::EventTitle) -> Self {
        Self(value.into_inner())
    }
}

impl TryFrom<EventDescription> for evops_models::EventDescription {
    type Error = ApiError;

    fn try_from(value: EventDescription) -> Result<Self, Self::Error> {
        Self::try_new(value.0).map_err(|e| ApiError::InvalidArgument(e.to_string()))
    }
}

impl From<evops_models::EventDescription> for EventDescription {
    fn from(value: evops_models::EventDescription) -> Self {
        Self(value.into_inner())
    }
}

impl TryFrom<UserDisplayName> for evops_models::UserDisplayName {
    type Error = ApiError;

    fn try_from(value: UserDisplayName) -> Result<Self, Self::Error> {
        Self::try_new(value.0).map_err(|e| ApiError::InvalidArgument(e.to_string()))
    }
}

impl From<evops_models::UserDisplayName> for UserDisplayName {
    fn from(value: evops_models::UserDisplayName) -> Self {
        Self(value.into_inner())
    }
}

impl From<UserId> for evops_models::UserId {
    fn from(value: UserId) -> Self {
        Self::new(value.0)
    }
}

impl From<evops_models::UserId> for UserId {
    fn from(value: evops_models::UserId) -> Self {
        Self(value.into_inner())
    }
}

impl From<TagId> for evops_models::TagId {
    fn from(value: TagId) -> Self {
        Self::new(value.0)
    }
}

impl From<evops_models::TagId> for TagId {
    fn from(value: evops_models::TagId) -> Self {
        Self(value.into_inner())
    }
}

impl TryFrom<TagName> for evops_models::TagName {
    type Error = ApiError;

    fn try_from(value: TagName) -> Result<Self, Self::Error> {
        Self::try_new(value.0).map_err(|e| ApiError::InvalidArgument(e.to_string()))
    }
}

impl From<evops_models::TagName> for TagName {
    fn from(value: evops_models::TagName) -> Self {
        Self(value.into_inner())
    }
}

impl TryFrom<TagAlias> for evops_models::TagAlias {
    type Error = ApiError;

    fn try_from(value: TagAlias) -> Result<Self, Self::Error> {
        Self::try_new(value.0).map_err(|e| ApiError::InvalidArgument(e.to_string()))
    }
}

impl From<evops_models::TagAlias> for TagAlias {
    fn from(value: evops_models::TagAlias) -> Self {
        Self(value.into_inner())
    }
}
