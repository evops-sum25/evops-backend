use std::time::SystemTime;

fn invalid_argument(message: impl ToString) -> tonic::Status {
    tonic::Status::invalid_argument(message.to_string())
}

impl TryFrom<crate::pb::EventServiceCreateRequest> for evops_types::EventServiceCreateRequest {
    type Error = tonic::Status;

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

impl From<evops_types::EventServiceCreateResponse> for crate::pb::EventServiceCreateResponse {
    fn from(value: evops_types::EventServiceCreateResponse) -> Self {
        Self {
            event: Some(value.event.into()),
        }
    }
}

impl TryFrom<crate::pb::TagServiceCreateRequest> for evops_types::TagServiceCreateRequest {
    type Error = tonic::Status;

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

impl From<evops_types::TagServiceCreateResponse> for crate::pb::TagServiceCreateResponse {
    fn from(value: evops_types::TagServiceCreateResponse) -> Self {
        Self {
            tag: Some(value.tag.into()),
        }
    }
}

impl TryFrom<crate::pb::UserServiceCreateRequest> for evops_types::UserServiceCreateRequest {
    type Error = tonic::Status;

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

impl From<evops_types::UserServiceCreateResponse> for crate::pb::UserServiceCreateResponse {
    fn from(value: evops_types::UserServiceCreateResponse) -> Self {
        Self {
            user: Some(value.user.into()),
        }
    }
}

impl TryFrom<crate::pb::NewEventForm> for evops_types::NewEventForm {
    type Error = tonic::Status;

    fn try_from(value: crate::pb::NewEventForm) -> Result<Self, Self::Error> {
        Ok(Self {
            author_id: evops_types::UserId::new({
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
                evops_types::EventTitle::try_new(value.title).map_err(self::invalid_argument)?
            },
            description: {
                evops_types::EventDescription::try_new(value.description)
                    .map_err(self::invalid_argument)?
            },
            tag_ids: Some({
                value
                    .tag_ids
                    .into_iter()
                    .map(|id| {
                        Ok::<_, Self::Error>(evops_types::TagId::new({
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

impl From<evops_types::Event> for crate::pb::Event {
    fn from(value: evops_types::Event) -> Self {
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

impl TryFrom<crate::pb::NewTagForm> for evops_types::NewTagForm {
    type Error = tonic::Status;

    fn try_from(value: crate::pb::NewTagForm) -> Result<Self, Self::Error> {
        Ok(Self {
            name: evops_types::TagName::try_new(value.name).map_err(self::invalid_argument)?,
            aliases: Some({
                value
                    .aliases
                    .into_iter()
                    .map(|a| {
                        Ok::<_, Self::Error>({
                            evops_types::TagAlias::try_new(a).map_err(self::invalid_argument)?
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?
            }),
        })
    }
}

impl From<evops_types::Tag> for crate::pb::Tag {
    fn from(value: evops_types::Tag) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name.into_inner(),
            aliases: value.aliases.into_iter().map(|a| a.into_inner()).collect(),
        }
    }
}

impl TryFrom<crate::pb::NewUserForm> for evops_types::NewUserForm {
    type Error = tonic::Status;

    fn try_from(value: crate::pb::NewUserForm) -> Result<Self, Self::Error> {
        Ok(Self {
            name: evops_types::UserName::try_new(value.name).map_err(self::invalid_argument)?,
            profile_picture_url: match value.profile_picture_url {
                Some(pp) => Some(pp.parse().map_err(self::invalid_argument)?),
                None => None,
            },
        })
    }
}

impl From<evops_types::User> for crate::pb::User {
    fn from(value: evops_types::User) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name.into_inner(),
            profile_picture_url: value.profile_picture_url.map(Into::into),
        }
    }
}
