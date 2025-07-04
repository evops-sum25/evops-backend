use evops_models::ApiError;

impl TryFrom<crate::types::NewEventForm> for evops_models::NewEventForm {
    type Error = ApiError;

    fn try_from(value: crate::types::NewEventForm) -> Result<Self, Self::Error> {
        Ok(Self {
            title: value.title.try_into()?,
            description: value.description.try_into()?,
            author_id: value.author_id.into(),
            tag_ids: value.tag_ids.unwrap_or_default().try_into()?,
            with_attendance: value.with_attendance,
        })
    }
}

impl TryFrom<crate::types::EventTagIds> for evops_models::EventTagIds {
    type Error = ApiError;

    fn try_from(value: crate::types::EventTagIds) -> Result<Self, Self::Error> {
        Self::try_new(value.0.into_iter().map(Into::into).collect())
            .map_err(|e| ApiError::InvalidArgument(e.to_string()))
    }
}

impl From<crate::types::EventImageId> for evops_models::EventImageId {
    fn from(value: crate::types::EventImageId) -> Self {
        Self::new(value.0)
    }
}

impl From<evops_models::EventImageId> for crate::types::EventImageId {
    fn from(value: evops_models::EventImageId) -> Self {
        Self(value.into_inner())
    }
}

impl From<evops_models::EventImageIds> for crate::types::EventImageIds {
    fn from(value: evops_models::EventImageIds) -> Self {
        Self(value.into_inner().into_iter().map(Into::into).collect())
    }
}

impl From<evops_models::EventTags> for crate::types::EventTags {
    fn from(value: evops_models::EventTags) -> Self {
        Self(value.into_inner().into_iter().map(Into::into).collect())
    }
}

impl From<evops_models::Event> for crate::types::Event {
    fn from(value: evops_models::Event) -> Self {
        Self {
            id: value.id.into(),
            title: value.title.into(),
            description: value.description.into(),
            author: value.author.into(),
            image_ids: value.image_ids.into(),
            tags: value.tags.into(),
            with_attendance: value.with_attendance,
            created_at: value.created_at,
            modified_at: value.modified_at,
        }
    }
}

impl TryFrom<crate::types::TagAliases> for evops_models::TagAliases {
    type Error = ApiError;

    fn try_from(value: crate::types::TagAliases) -> Result<Self, Self::Error> {
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

impl From<evops_models::TagAliases> for crate::types::TagAliases {
    fn from(value: evops_models::TagAliases) -> Self {
        Self(value.into_inner().into_iter().map(Into::into).collect())
    }
}

impl TryFrom<crate::types::NewTagForm> for evops_models::NewTagForm {
    type Error = ApiError;

    fn try_from(value: crate::types::NewTagForm) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.try_into()?,
            aliases: value.aliases.unwrap_or_default().try_into()?,
        })
    }
}

impl From<evops_models::Tag> for crate::types::Tag {
    fn from(value: evops_models::Tag) -> Self {
        Self {
            id: value.id.into(),
            name: value.name.into(),
            aliases: value.aliases.into(),
        }
    }
}

impl TryFrom<crate::types::NewUserForm> for evops_models::NewUserForm {
    type Error = ApiError;

    fn try_from(value: crate::types::NewUserForm) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.try_into()?,
        })
    }
}

impl From<evops_models::User> for crate::types::User {
    fn from(value: evops_models::User) -> Self {
        Self {
            id: value.id.into(),
            name: value.name.into(),
        }
    }
}

impl From<evops_models::EventId> for crate::types::EventId {
    fn from(value: evops_models::EventId) -> Self {
        Self(value.into_inner())
    }
}

impl From<crate::types::EventId> for evops_models::EventId {
    fn from(value: crate::types::EventId) -> Self {
        Self::new(value.0)
    }
}

impl From<evops_models::PgLimit> for crate::types::PgLimit {
    fn from(value: evops_models::PgLimit) -> Self {
        Self(value.into_inner())
    }
}

impl TryFrom<crate::types::PgLimit> for evops_models::PgLimit {
    type Error = ApiError;

    fn try_from(value: crate::types::PgLimit) -> Result<Self, Self::Error> {
        Self::try_new(value.0).map_err(|e| ApiError::InvalidArgument(e.to_string()))
    }
}

impl TryFrom<crate::types::EventTitle> for evops_models::EventTitle {
    type Error = ApiError;

    fn try_from(value: crate::types::EventTitle) -> Result<Self, Self::Error> {
        Self::try_new(value.0).map_err(|e| ApiError::InvalidArgument(e.to_string()))
    }
}

impl From<evops_models::EventTitle> for crate::types::EventTitle {
    fn from(value: evops_models::EventTitle) -> Self {
        Self(value.into_inner())
    }
}

impl TryFrom<crate::types::EventDescription> for evops_models::EventDescription {
    type Error = ApiError;

    fn try_from(value: crate::types::EventDescription) -> Result<Self, Self::Error> {
        Self::try_new(value.0).map_err(|e| ApiError::InvalidArgument(e.to_string()))
    }
}

impl From<evops_models::EventDescription> for crate::types::EventDescription {
    fn from(value: evops_models::EventDescription) -> Self {
        Self(value.into_inner())
    }
}

impl TryFrom<crate::types::UserName> for evops_models::UserName {
    type Error = ApiError;

    fn try_from(value: crate::types::UserName) -> Result<Self, Self::Error> {
        Self::try_new(value.0).map_err(|e| ApiError::InvalidArgument(e.to_string()))
    }
}

impl From<evops_models::UserName> for crate::types::UserName {
    fn from(value: evops_models::UserName) -> Self {
        Self(value.into_inner())
    }
}

impl From<crate::types::UserId> for evops_models::UserId {
    fn from(value: crate::types::UserId) -> Self {
        Self::new(value.0)
    }
}

impl From<evops_models::UserId> for crate::types::UserId {
    fn from(value: evops_models::UserId) -> Self {
        Self(value.into_inner())
    }
}

impl From<crate::types::TagId> for evops_models::TagId {
    fn from(value: crate::types::TagId) -> Self {
        Self::new(value.0)
    }
}

impl From<evops_models::TagId> for crate::types::TagId {
    fn from(value: evops_models::TagId) -> Self {
        Self(value.into_inner())
    }
}

impl TryFrom<crate::types::TagName> for evops_models::TagName {
    type Error = ApiError;

    fn try_from(value: crate::types::TagName) -> Result<Self, Self::Error> {
        Self::try_new(value.0).map_err(|e| ApiError::InvalidArgument(e.to_string()))
    }
}

impl From<evops_models::TagName> for crate::types::TagName {
    fn from(value: evops_models::TagName) -> Self {
        Self(value.into_inner())
    }
}

impl TryFrom<crate::types::TagAlias> for evops_models::TagAlias {
    type Error = ApiError;

    fn try_from(value: crate::types::TagAlias) -> Result<Self, Self::Error> {
        Self::try_new(value.0).map_err(|e| ApiError::InvalidArgument(e.to_string()))
    }
}

impl From<evops_models::TagAlias> for crate::types::TagAlias {
    fn from(value: evops_models::TagAlias) -> Self {
        Self(value.into_inner())
    }
}
