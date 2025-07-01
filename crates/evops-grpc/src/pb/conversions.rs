use std::time::SystemTime;

use uuid::Uuid;

use evops_models::ApiError;

impl TryFrom<crate::pb::NewEventForm> for evops_models::NewEventForm {
    type Error = ApiError;

    fn try_from(value: crate::pb::NewEventForm) -> Result<Self, Self::Error> {
        Ok(Self {
            author_id: evops_models::UserId::new({
                value
                    .author_id
                    .parse::<Uuid>()
                    .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
            }),
            title: {
                evops_models::EventTitle::try_new(value.title)
                    .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
            },
            description: {
                evops_models::EventDescription::try_new(value.description)
                    .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
            },
            tag_ids: {
                evops_models::EventTagIds::try_new({
                    value
                        .tag_ids
                        .into_iter()
                        .map(|id| {
                            Ok::<_, Self::Error>(evops_models::TagId::new({
                                id.parse::<Uuid>()
                                    .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
                            }))
                        })
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
                })
                .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
            },
            with_attendance: value.with_attendance,
        })
    }
}

impl From<evops_models::Event> for crate::pb::Event {
    fn from(value: evops_models::Event) -> Self {
        Self {
            id: value.id.to_string(),
            title: value.title.into_inner(),
            description: value.description.into_inner(),
            author: Some(value.author.into()),
            image_ids: {
                value
                    .image_ids
                    .into_inner()
                    .into_iter()
                    .map(|img_id| img_id.into_inner().into())
                    .collect()
            },
            tags: {
                value
                    .tags
                    .into_inner()
                    .into_iter()
                    .map(Into::into)
                    .collect()
            },
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
            name: {
                evops_models::TagName::try_new(value.name)
                    .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
            },
            aliases: {
                evops_models::TagAliases::try_new({
                    value
                        .aliases
                        .into_iter()
                        .map(|tag_alias| {
                            Ok::<_, Self::Error>({
                                evops_models::TagAlias::try_new(tag_alias)
                                    .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()?
                })
                .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
            },
        })
    }
}

impl From<evops_models::Tag> for crate::pb::Tag {
    fn from(value: evops_models::Tag) -> Self {
        Self {
            id: value.id.to_string(),
            name: value.name.into_inner(),
            aliases: {
                value
                    .aliases
                    .into_inner()
                    .into_iter()
                    .map(|a| a.into_inner())
                    .collect()
            },
        }
    }
}

impl TryFrom<crate::pb::NewUserForm> for evops_models::NewUserForm {
    type Error = ApiError;

    fn try_from(value: crate::pb::NewUserForm) -> Result<Self, Self::Error> {
        Ok(Self {
            name: {
                evops_models::UserName::try_new(value.name)
                    .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
            },
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
