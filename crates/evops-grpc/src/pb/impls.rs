use std::time::SystemTime;

use uuid::Uuid;

use evops_models::{ApiError, ApiResult};

use crate::pb::{
    AuthTokens, Event, NewEventForm, NewLanguageForm, NewTagForm, NewUserForm, Tag,
    UpdateEventForm, User,
};

impl TryFrom<NewLanguageForm> for evops_models::NewLanguageForm {
    type Error = ApiError;

    fn try_from(value: NewLanguageForm) -> Result<Self, Self::Error> {
        Ok(Self {
            name: {
                evops_models::LanguageName::try_new(value.name)
                    .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
            },
        })
    }
}

impl From<evops_models::AuthTokens> for AuthTokens {
    fn from(value: evops_models::AuthTokens) -> Self {
        Self {
            access: value.access.into_inner(),
            refresh: value.refresh.into_inner(),
        }
    }
}

impl TryFrom<UpdateEventForm> for evops_models::UpdateEventForm {
    type Error = ApiError;

    fn try_from(value: UpdateEventForm) -> Result<Self, Self::Error> {
        Ok(Self {
            title: match value.title {
                Some(title) => Some({
                    evops_models::EventTitle::try_new(title)
                        .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
                }),
                None => None,
            },
            description: match value.description {
                Some(description) => Some({
                    evops_models::EventDescription::try_new(description)
                        .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
                }),
                None => None,
            },
            tag_ids: match value.tag_ids {
                Some(tag_ids) => Some(
                    evops_models::EventTagIds::try_new({
                        tag_ids
                            .replace_with
                            .into_iter()
                            .map(|tag_id| {
                                ApiResult::Ok(evops_models::TagId::new({
                                    tag_id
                                        .parse::<Uuid>()
                                        .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
                                }))
                            })
                            .collect::<Result<_, _>>()?
                    })
                    .map_err(|e| ApiError::InvalidArgument(e.to_string()))?,
                ),
                None => None,
            },
        })
    }
}

impl TryFrom<NewEventForm> for evops_models::NewEventForm {
    type Error = ApiError;

    fn try_from(value: NewEventForm) -> Result<Self, Self::Error> {
        Ok(Self {
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
        })
    }
}

impl From<evops_models::Event> for Event {
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
            created_at: Some(SystemTime::from(value.created_at).into()),
            modified_at: Some(SystemTime::from(value.modified_at).into()),
        }
    }
}

impl TryFrom<NewTagForm> for evops_models::NewTagForm {
    type Error = ApiError;

    fn try_from(value: NewTagForm) -> Result<Self, Self::Error> {
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

impl From<evops_models::Tag> for Tag {
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

impl TryFrom<NewUserForm> for evops_models::NewUserForm {
    type Error = ApiError;

    fn try_from(value: NewUserForm) -> Result<Self, Self::Error> {
        Ok(Self {
            login: {
                evops_models::UserLogin::try_new(value.login)
                    .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
            },
            display_name: {
                evops_models::UserDisplayName::try_new(value.display_name)
                    .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
            },
            password: {
                evops_models::UserPassword::try_new(value.password)
                    .map_err(|e| ApiError::InvalidArgument(e.to_string()))?
            },
        })
    }
}

impl From<evops_models::User> for User {
    fn from(value: evops_models::User) -> Self {
        Self {
            id: value.id.to_string(),
            login: value.login.into_inner(),
            display_name: value.display_name.into_inner(),
        }
    }
}
