impl From<crate::types::UserServiceListRequest> for evops_types::UserServiceListRequest {
    fn from(_value: crate::types::UserServiceListRequest) -> Self {
        Self
    }
}

impl From<evops_types::UserServiceListResponse> for crate::types::UserServiceListResponse {
    fn from(value: evops_types::UserServiceListResponse) -> Self {
        Self {
            users: value.users.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<crate::types::EventServiceCreateRequest> for evops_types::EventServiceCreateRequest {
    type Error = crate::error::UnprocessableEntityError;

    fn try_from(value: crate::types::EventServiceCreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            form: value.form.try_into()?,
        })
    }
}

impl From<evops_types::EventServiceCreateResponse> for crate::types::EventServiceCreateResponse {
    fn from(value: evops_types::EventServiceCreateResponse) -> Self {
        Self {
            event: value.event.into(),
        }
    }
}

impl TryFrom<crate::types::TagServiceCreateRequest> for evops_types::TagServiceCreateRequest {
    type Error = crate::error::UnprocessableEntityError;

    fn try_from(value: crate::types::TagServiceCreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            form: value.form.try_into()?,
        })
    }
}

impl From<evops_types::TagServiceCreateResponse> for crate::types::TagServiceCreateResponse {
    fn from(value: evops_types::TagServiceCreateResponse) -> Self {
        Self {
            tag: value.tag.into(),
        }
    }
}

impl TryFrom<crate::types::UserServiceCreateRequest> for evops_types::UserServiceCreateRequest {
    type Error = crate::error::UnprocessableEntityError;

    fn try_from(value: crate::types::UserServiceCreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            form: value.form.try_into()?,
        })
    }
}

impl From<evops_types::UserServiceCreateResponse> for crate::types::UserServiceCreateResponse {
    fn from(value: evops_types::UserServiceCreateResponse) -> Self {
        Self {
            user: value.user.into(),
        }
    }
}

impl TryFrom<crate::types::NewEventForm> for evops_types::NewEventForm {
    type Error = crate::error::UnprocessableEntityError;

    fn try_from(value: crate::types::NewEventForm) -> Result<Self, Self::Error> {
        Ok(Self {
            author_id: value.author_id.into(),
            image_urls: value.image_urls,
            title: value.title.try_into()?,
            description: value.description.try_into()?,
            tag_ids: {
                value
                    .tag_ids
                    .map(|t| t.into_iter().map(Into::into).collect())
            },
            with_attendance: value.with_attendance,
        })
    }
}

impl From<evops_types::Event> for crate::types::Event {
    fn from(value: evops_types::Event) -> Self {
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

impl TryFrom<crate::types::NewTagForm> for evops_types::NewTagForm {
    type Error = crate::error::UnprocessableEntityError;

    fn try_from(value: crate::types::NewTagForm) -> Result<Self, Self::Error> {
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

impl From<evops_types::Tag> for crate::types::Tag {
    fn from(value: evops_types::Tag) -> Self {
        Self {
            id: value.id.into(),
            name: value.name.into(),
            aliases: value.aliases.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<crate::types::NewUserForm> for evops_types::NewUserForm {
    type Error = crate::error::UnprocessableEntityError;

    fn try_from(value: crate::types::NewUserForm) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name.try_into()?,
            profile_picture_url: value.profile_picture_url,
        })
    }
}

impl From<evops_types::User> for crate::types::User {
    fn from(value: evops_types::User) -> Self {
        Self {
            id: value.id.into(),
            name: value.name.into(),
            profile_picture_url: value.profile_picture_url,
        }
    }
}

impl From<evops_types::EventId> for crate::types::EventId {
    fn from(value: evops_types::EventId) -> Self {
        Self(value.into_inner())
    }
}

impl From<crate::types::EventId> for evops_types::EventId {
    fn from(value: crate::types::EventId) -> Self {
        Self::new(value.0)
    }
}

impl TryFrom<crate::types::EventTitle> for evops_types::EventTitle {
    type Error = crate::error::UnprocessableEntityError;

    fn try_from(value: crate::types::EventTitle) -> Result<Self, Self::Error> {
        Ok(Self::try_new(value.0)?)
    }
}

impl From<evops_types::EventTitle> for crate::types::EventTitle {
    fn from(value: evops_types::EventTitle) -> Self {
        Self(value.into_inner())
    }
}

impl TryFrom<crate::types::EventDescription> for evops_types::EventDescription {
    type Error = crate::error::UnprocessableEntityError;

    fn try_from(value: crate::types::EventDescription) -> Result<Self, Self::Error> {
        Ok(Self::try_new(value.0)?)
    }
}

impl From<evops_types::EventDescription> for crate::types::EventDescription {
    fn from(value: evops_types::EventDescription) -> Self {
        Self(value.into_inner())
    }
}

impl TryFrom<crate::types::UserName> for evops_types::UserName {
    type Error = crate::error::UnprocessableEntityError;

    fn try_from(value: crate::types::UserName) -> Result<Self, Self::Error> {
        Ok(Self::try_new(value.0)?)
    }
}

impl From<evops_types::UserName> for crate::types::UserName {
    fn from(value: evops_types::UserName) -> Self {
        Self(value.into_inner())
    }
}

impl From<crate::types::UserId> for evops_types::UserId {
    fn from(value: crate::types::UserId) -> Self {
        Self::new(value.0)
    }
}

impl From<evops_types::UserId> for crate::types::UserId {
    fn from(value: evops_types::UserId) -> Self {
        Self(value.into_inner())
    }
}

impl From<crate::types::TagId> for evops_types::TagId {
    fn from(value: crate::types::TagId) -> Self {
        Self::new(value.0)
    }
}

impl From<evops_types::TagId> for crate::types::TagId {
    fn from(value: evops_types::TagId) -> Self {
        Self(value.into_inner())
    }
}

impl TryFrom<crate::types::TagName> for evops_types::TagName {
    type Error = crate::error::UnprocessableEntityError;

    fn try_from(value: crate::types::TagName) -> Result<Self, Self::Error> {
        Ok(Self::try_new(value.0)?)
    }
}

impl From<evops_types::TagName> for crate::types::TagName {
    fn from(value: evops_types::TagName) -> Self {
        Self(value.into_inner())
    }
}

impl TryFrom<crate::types::TagAlias> for evops_types::TagAlias {
    type Error = crate::error::UnprocessableEntityError;

    fn try_from(value: crate::types::TagAlias) -> Result<Self, Self::Error> {
        Ok(Self::try_new(value.0)?)
    }
}

impl From<evops_types::TagAlias> for crate::types::TagAlias {
    fn from(value: evops_types::TagAlias) -> Self {
        Self(value.into_inner())
    }
}
