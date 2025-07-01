use evops_models::ApiError;

fn invalid_argument(e: &impl ToString) -> ApiError {
    ApiError::InvalidArgument(e.to_string())
}

impl From<crate::types::EventServiceFindRequest> for evops_models::EventServiceFindRequest {
    fn from(value: crate::types::EventServiceFindRequest) -> Self {
        Self {
            id: value.id.into(),
        }
    }
}

impl From<evops_models::EventServiceFindResponse> for crate::types::EventServiceFindResponse {
    fn from(value: evops_models::EventServiceFindResponse) -> Self {
        Self {
            event: value.event.into(),
        }
    }
}

impl TryFrom<crate::types::EventServiceListRequest> for evops_models::EventServiceListRequest {
    type Error = ApiError;

    fn try_from(value: crate::types::EventServiceListRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            last_id: value.last_id.map(Into::into),
            limit: match value.limit {
                Some(l) => Some(l.try_into().map_err(|e| self::invalid_argument(&e))?),
                _ => None,
            },
        })
    }
}

impl From<evops_models::EventServiceListResponse> for crate::types::EventServiceListResponse {
    fn from(value: evops_models::EventServiceListResponse) -> Self {
        Self {
            events: value.events.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<crate::types::EventServiceCreateRequest> for evops_models::EventServiceCreateRequest {
    type Error = ApiError;

    fn try_from(value: crate::types::EventServiceCreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            form: value.form.try_into()?,
        })
    }
}

impl From<evops_models::EventServiceCreateResponse> for crate::types::EventServiceCreateResponse {
    fn from(value: evops_models::EventServiceCreateResponse) -> Self {
        Self {
            event: value.event.into(),
        }
    }
}

impl From<crate::types::TagServiceFindRequest> for evops_models::TagServiceFindRequest {
    fn from(value: crate::types::TagServiceFindRequest) -> Self {
        Self {
            id: value.id.into(),
        }
    }
}

impl From<evops_models::TagServiceFindResponse> for crate::types::TagServiceFindResponse {
    fn from(value: evops_models::TagServiceFindResponse) -> Self {
        Self {
            tag: value.tag.into(),
        }
    }
}

impl TryFrom<crate::types::TagServiceListRequest> for evops_models::TagServiceListRequest {
    type Error = ApiError;

    fn try_from(value: crate::types::TagServiceListRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            last_id: value.last_id.map(Into::into),
            limit: match value.limit {
                Some(l) => Some(l.try_into().map_err(|e| self::invalid_argument(&e))?),
                _ => None,
            },
        })
    }
}

impl From<evops_models::TagServiceListResponse> for crate::types::TagServiceListResponse {
    fn from(value: evops_models::TagServiceListResponse) -> Self {
        Self {
            tags: value.tags.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<crate::types::TagServiceCreateRequest> for evops_models::TagServiceCreateRequest {
    type Error = ApiError;

    fn try_from(value: crate::types::TagServiceCreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            form: value.form.try_into()?,
        })
    }
}

impl From<evops_models::TagServiceCreateResponse> for crate::types::TagServiceCreateResponse {
    fn from(value: evops_models::TagServiceCreateResponse) -> Self {
        Self {
            tag: value.tag.into(),
        }
    }
}

impl From<crate::types::UserServiceFindRequest> for evops_models::UserServiceFindRequest {
    fn from(value: crate::types::UserServiceFindRequest) -> Self {
        Self {
            id: value.id.into(),
        }
    }
}

impl From<evops_models::UserServiceFindResponse> for crate::types::UserServiceFindResponse {
    fn from(value: evops_models::UserServiceFindResponse) -> Self {
        Self {
            user: value.user.into(),
        }
    }
}

impl From<crate::types::UserServiceListRequest> for evops_models::UserServiceListRequest {
    fn from(_value: crate::types::UserServiceListRequest) -> Self {
        Self
    }
}

impl From<evops_models::UserServiceListResponse> for crate::types::UserServiceListResponse {
    fn from(value: evops_models::UserServiceListResponse) -> Self {
        Self {
            users: value.users.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<crate::types::UserServiceCreateRequest> for evops_models::UserServiceCreateRequest {
    type Error = ApiError;

    fn try_from(value: crate::types::UserServiceCreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            form: value.form.try_into()?,
        })
    }
}

impl From<evops_models::UserServiceCreateResponse> for crate::types::UserServiceCreateResponse {
    fn from(value: evops_models::UserServiceCreateResponse) -> Self {
        Self {
            user: value.user.into(),
        }
    }
}

impl TryFrom<crate::types::NewEventForm> for evops_models::NewEventForm {
    type Error = ApiError;

    fn try_from(value: crate::types::NewEventForm) -> Result<Self, Self::Error> {
        Ok(Self {
            author_id: value.author_id.into(),
            image_urls: todo!(),
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

impl From<evops_models::Event> for crate::types::Event {
    fn from(value: evops_models::Event) -> Self {
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

impl TryFrom<crate::types::NewTagForm> for evops_models::NewTagForm {
    type Error = ApiError;

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

impl From<evops_models::Tag> for crate::types::Tag {
    fn from(value: evops_models::Tag) -> Self {
        Self {
            id: value.id.into(),
            name: value.name.into(),
            aliases: value.aliases.into_iter().map(Into::into).collect(),
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
        Self::try_new(value.0).map_err(|e| self::invalid_argument(&e))
    }
}

impl TryFrom<crate::types::EventTitle> for evops_models::EventTitle {
    type Error = ApiError;

    fn try_from(value: crate::types::EventTitle) -> Result<Self, Self::Error> {
        Self::try_new(value.0).map_err(|e| self::invalid_argument(&e))
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
        Self::try_new(value.0).map_err(|e| self::invalid_argument(&e))
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
        Self::try_new(value.0).map_err(|e| self::invalid_argument(&e))
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
        Self::try_new(value.0).map_err(|e| self::invalid_argument(&e))
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
        Self::try_new(value.0).map_err(|e| self::invalid_argument(&e))
    }
}

impl From<evops_models::TagAlias> for crate::types::TagAlias {
    fn from(value: evops_models::TagAlias) -> Self {
        Self(value.into_inner())
    }
}
