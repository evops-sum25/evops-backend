use std::time::SystemTime;

use evops_models::ApiError;

fn invalid_argument(message: impl ToString) -> ApiError {
    ApiError::InvalidArgument(message.to_string())
}

impl TryFrom<crate::pb::EventServiceFindRequest> for evops_models::EventServiceFindRequest {
    type Error = ApiError;

    fn try_from(value: crate::pb::EventServiceFindRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: evops_models::EventId::new(value.id.parse().map_err(self::invalid_argument)?),
        })
    }
}

impl From<evops_models::EventServiceFindResponse> for crate::pb::EventServiceFindResponse {
    fn from(value: evops_models::EventServiceFindResponse) -> Self {
        Self {
            event: Some(value.event.into()),
        }
    }
}

impl TryFrom<crate::pb::EventServiceListRequest> for evops_models::EventServiceListRequest {
    type Error = ApiError;

    fn try_from(value: crate::pb::EventServiceListRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            last_id: match value.last_id {
                Some(id) => Some(evops_models::EventId::new(
                    id.parse().map_err(self::invalid_argument)?,
                )),
                _ => None,
            },
            limit: match value.limit {
                Some(l) => Some(evops_models::PgLimit::try_new(l).map_err(self::invalid_argument)?),
                _ => None,
            },
        })
    }
}

impl From<evops_models::EventServiceListResponse> for crate::pb::EventServiceListResponse {
    fn from(value: evops_models::EventServiceListResponse) -> Self {
        Self {
            events: value.events.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<crate::pb::EventServiceCreateRequest> for evops_models::EventServiceCreateRequest {
    type Error = ApiError;

    fn try_from(value: crate::pb::EventServiceCreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            form: {
                value
                    .form
                    .ok_or(self::invalid_argument({
                        "EventServiceCreateRequest.form must be set."
                    }))?
                    .try_into()?
            },
        })
    }
}

impl From<evops_models::EventServiceCreateResponse> for crate::pb::EventServiceCreateResponse {
    fn from(value: evops_models::EventServiceCreateResponse) -> Self {
        Self {
            event: Some(value.event.into()),
        }
    }
}

impl TryFrom<crate::pb::TagServiceFindRequest> for evops_models::TagServiceFindRequest {
    type Error = ApiError;

    fn try_from(value: crate::pb::TagServiceFindRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: evops_models::TagId::new(value.id.parse().map_err(self::invalid_argument)?),
        })
    }
}

impl From<evops_models::TagServiceFindResponse> for crate::pb::TagServiceFindResponse {
    fn from(value: evops_models::TagServiceFindResponse) -> Self {
        Self {
            tag: Some(value.tag.into()),
        }
    }
}

impl TryFrom<crate::pb::TagServiceListRequest> for evops_models::TagServiceListRequest {
    type Error = ApiError;

    fn try_from(value: crate::pb::TagServiceListRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            last_id: match value.last_id {
                Some(id) => Some(evops_models::TagId::new(
                    id.parse().map_err(self::invalid_argument)?,
                )),
                _ => None,
            },
            limit: match value.limit {
                Some(l) => Some(evops_models::PgLimit::try_new(l).map_err(self::invalid_argument)?),
                _ => None,
            },
        })
    }
}

impl From<evops_models::TagServiceListResponse> for crate::pb::TagServiceListResponse {
    fn from(value: evops_models::TagServiceListResponse) -> Self {
        Self {
            tags: value.tags.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<crate::pb::TagServiceCreateRequest> for evops_models::TagServiceCreateRequest {
    type Error = ApiError;

    fn try_from(value: crate::pb::TagServiceCreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            form: {
                value
                    .form
                    .ok_or(self::invalid_argument({
                        "TagServiceCreateRequest.form must be set."
                    }))?
                    .try_into()?
            },
        })
    }
}

impl From<evops_models::TagServiceCreateResponse> for crate::pb::TagServiceCreateResponse {
    fn from(value: evops_models::TagServiceCreateResponse) -> Self {
        Self {
            tag: Some(value.tag.into()),
        }
    }
}

impl TryFrom<crate::pb::UserServiceFindRequest> for evops_models::UserServiceFindRequest {
    type Error = ApiError;

    fn try_from(value: crate::pb::UserServiceFindRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: evops_models::UserId::new(value.id.parse().map_err(self::invalid_argument)?),
        })
    }
}

impl From<evops_models::UserServiceFindResponse> for crate::pb::UserServiceFindResponse {
    fn from(value: evops_models::UserServiceFindResponse) -> Self {
        Self {
            user: Some(value.user.into()),
        }
    }
}

impl From<crate::pb::UserServiceListRequest> for evops_models::UserServiceListRequest {
    fn from(_value: crate::pb::UserServiceListRequest) -> Self {
        Self
    }
}

impl From<evops_models::UserServiceListResponse> for crate::pb::UserServiceListResponse {
    fn from(value: evops_models::UserServiceListResponse) -> Self {
        Self {
            users: value.users.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<crate::pb::UserServiceCreateRequest> for evops_models::UserServiceCreateRequest {
    type Error = ApiError;

    fn try_from(value: crate::pb::UserServiceCreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            form: {
                value
                    .form
                    .ok_or(self::invalid_argument({
                        "TagServiceCreateRequest.form must be set."
                    }))?
                    .try_into()?
            },
        })
    }
}

impl From<evops_models::UserServiceCreateResponse> for crate::pb::UserServiceCreateResponse {
    fn from(value: evops_models::UserServiceCreateResponse) -> Self {
        Self {
            user: Some(value.user.into()),
        }
    }
}

impl TryFrom<crate::pb::NewEventForm> for evops_models::NewEventForm {
    type Error = ApiError;

    fn try_from(value: crate::pb::NewEventForm) -> Result<Self, Self::Error> {
        Ok(Self {
            author_id: evops_models::UserId::new({
                value.author_id.parse().map_err(self::invalid_argument)?
            }),
            image_urls: Some({
                value
                    .image_urls
                    .into_iter()
                    .map(|s| s.parse())
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(self::invalid_argument)?
            }),
            title: {
                evops_models::EventTitle::try_new(value.title).map_err(self::invalid_argument)?
            },
            description: {
                evops_models::EventDescription::try_new(value.description)
                    .map_err(self::invalid_argument)?
            },
            tag_ids: Some({
                value
                    .tag_ids
                    .into_iter()
                    .map(|id| {
                        Ok::<_, Self::Error>(evops_models::TagId::new({
                            id.parse().map_err(self::invalid_argument)?
                        }))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(self::invalid_argument)?
            }),
            with_attendance: value.with_attendance,
        })
    }
}

impl From<evops_models::Event> for crate::pb::Event {
    fn from(value: evops_models::Event) -> Self {
        Self {
            id: value.id.to_string(),
            author: Some(value.author.into()),
            image_urls: value.image_urls.into_iter().map(Into::into).collect(),
            title: value.title.into_inner(),
            description: value.description.into_inner(),
            tags: value.tags.into_iter().map(Into::into).collect(),
            with_attendance: value.with_attendance,
            created_at: Some(SystemTime::from(value.created_at).into()),
            modified_at: Some(SystemTime::from(value.modified_at).into()),
        }
    }
}

impl TryFrom<crate::pb::NewTagForm> for evops_models::NewTagForm {
    type Error = ApiError;

    fn try_from(value: crate::pb::NewTagForm) -> Result<Self, Self::Error> {
        Ok(Self {
            name: evops_models::TagName::try_new(value.name).map_err(self::invalid_argument)?,
            aliases: Some({
                value
                    .aliases
                    .into_iter()
                    .map(|a| {
                        Ok::<_, Self::Error>({
                            evops_models::TagAlias::try_new(a).map_err(self::invalid_argument)?
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?
            }),
        })
    }
}

impl From<evops_models::Tag> for crate::pb::Tag {
    fn from(value: evops_models::Tag) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name.into_inner(),
            aliases: value.aliases.into_iter().map(|a| a.into_inner()).collect(),
        }
    }
}

impl TryFrom<crate::pb::NewUserForm> for evops_models::NewUserForm {
    type Error = ApiError;

    fn try_from(value: crate::pb::NewUserForm) -> Result<Self, Self::Error> {
        Ok(Self {
            name: evops_models::UserName::try_new(value.name).map_err(self::invalid_argument)?,
        })
    }
}

impl From<evops_models::User> for crate::pb::User {
    fn from(value: evops_models::User) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name.into_inner(),
        }
    }
}
