use std::time::SystemTime;

use evops_models::ApiError;

fn invalid_argument(message: impl ToString) -> ApiError {
    ApiError::InvalidArgument(message.to_string())
}

impl TryFrom<crate::api::EventServiceFindRequest> for evops_models::EventServiceFindRequest {
    type Error = ApiError;

    fn try_from(value: crate::api::EventServiceFindRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: evops_models::EventId::new(value.id.parse().map_err(self::invalid_argument)?),
        })
    }
}

impl From<evops_models::EventServiceFindResponse> for crate::api::EventServiceFindResponse {
    fn from(value: evops_models::EventServiceFindResponse) -> Self {
        Self {
            event: Some(value.event.into()),
        }
    }
}

impl From<crate::api::EventServiceListRequest> for evops_models::EventServiceListRequest {
    fn from(_value: crate::api::EventServiceListRequest) -> Self {
        Self
    }
}

impl From<evops_models::EventServiceListResponse> for crate::api::EventServiceListResponse {
    fn from(value: evops_models::EventServiceListResponse) -> Self {
        Self {
            events: value.events.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<crate::api::EventServiceCreateRequest> for evops_models::EventServiceCreateRequest {
    type Error = ApiError;

    fn try_from(value: crate::api::EventServiceCreateRequest) -> Result<Self, Self::Error> {
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

impl From<evops_models::EventServiceCreateResponse> for crate::api::EventServiceCreateResponse {
    fn from(value: evops_models::EventServiceCreateResponse) -> Self {
        Self {
            event: Some(value.event.into()),
        }
    }
}

impl TryFrom<crate::api::TagServiceFindRequest> for evops_models::TagServiceFindRequest {
    type Error = ApiError;

    fn try_from(value: crate::api::TagServiceFindRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: evops_models::TagId::new(value.id.parse().map_err(self::invalid_argument)?),
        })
    }
}

impl From<evops_models::TagServiceFindResponse> for crate::api::TagServiceFindResponse {
    fn from(value: evops_models::TagServiceFindResponse) -> Self {
        Self {
            tag: Some(value.tag.into()),
        }
    }
}

impl From<crate::api::TagServiceListRequest> for evops_models::TagServiceListRequest {
    fn from(_value: crate::api::TagServiceListRequest) -> Self {
        Self
    }
}

impl From<evops_models::TagServiceListResponse> for crate::api::TagServiceListResponse {
    fn from(value: evops_models::TagServiceListResponse) -> Self {
        Self {
            tags: value.tags.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<crate::api::TagServiceCreateRequest> for evops_models::TagServiceCreateRequest {
    type Error = ApiError;

    fn try_from(value: crate::api::TagServiceCreateRequest) -> Result<Self, Self::Error> {
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

impl From<evops_models::TagServiceCreateResponse> for crate::api::TagServiceCreateResponse {
    fn from(value: evops_models::TagServiceCreateResponse) -> Self {
        Self {
            tag: Some(value.tag.into()),
        }
    }
}

impl TryFrom<crate::api::UserServiceFindRequest> for evops_models::UserServiceFindRequest {
    type Error = ApiError;

    fn try_from(value: crate::api::UserServiceFindRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: evops_models::UserId::new(value.id.parse().map_err(self::invalid_argument)?),
        })
    }
}

impl From<evops_models::UserServiceFindResponse> for crate::api::UserServiceFindResponse {
    fn from(value: evops_models::UserServiceFindResponse) -> Self {
        Self {
            user: Some(value.user.into()),
        }
    }
}

impl From<crate::api::UserServiceListRequest> for evops_models::UserServiceListRequest {
    fn from(_value: crate::api::UserServiceListRequest) -> Self {
        Self
    }
}

impl From<evops_models::UserServiceListResponse> for crate::api::UserServiceListResponse {
    fn from(value: evops_models::UserServiceListResponse) -> Self {
        Self {
            users: value.users.into_iter().map(Into::into).collect(),
        }
    }
}

impl TryFrom<crate::api::UserServiceCreateRequest> for evops_models::UserServiceCreateRequest {
    type Error = ApiError;

    fn try_from(value: crate::api::UserServiceCreateRequest) -> Result<Self, Self::Error> {
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

impl From<evops_models::UserServiceCreateResponse> for crate::api::UserServiceCreateResponse {
    fn from(value: evops_models::UserServiceCreateResponse) -> Self {
        Self {
            user: Some(value.user.into()),
        }
    }
}

impl TryFrom<crate::api::NewEventForm> for evops_models::NewEventForm {
    type Error = ApiError;

    fn try_from(value: crate::api::NewEventForm) -> Result<Self, Self::Error> {
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

impl From<evops_models::Event> for crate::api::Event {
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

impl TryFrom<crate::api::NewTagForm> for evops_models::NewTagForm {
    type Error = ApiError;

    fn try_from(value: crate::api::NewTagForm) -> Result<Self, Self::Error> {
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

impl From<evops_models::Tag> for crate::api::Tag {
    fn from(value: evops_models::Tag) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name.into_inner(),
            aliases: value.aliases.into_iter().map(|a| a.into_inner()).collect(),
        }
    }
}

impl TryFrom<crate::api::NewUserForm> for evops_models::NewUserForm {
    type Error = ApiError;

    fn try_from(value: crate::api::NewUserForm) -> Result<Self, Self::Error> {
        Ok(Self {
            name: evops_models::UserName::try_new(value.name).map_err(self::invalid_argument)?,
            profile_picture_url: match value.profile_picture_url {
                Some(pp) => Some(pp.parse().map_err(self::invalid_argument)?),
                None => None,
            },
        })
    }
}

impl From<evops_models::User> for crate::api::User {
    fn from(value: evops_models::User) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name.into_inner(),
            profile_picture_url: value.profile_picture_url.map(Into::into),
        }
    }
}
