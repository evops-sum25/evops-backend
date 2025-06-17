use chrono::{DateTime, Utc};
use nutype::nutype;
use thiserror::Error;
use url::Url;
use uuid::Uuid;

#[derive(Debug)]
pub struct CreateEventRequest {
    pub author_id: crate::UserId,
    pub image_urls: Option<Vec<Url>>,
    pub title: crate::EventTitle,
    pub description: crate::EventDescription,
    pub tag_ids: Option<Vec<crate::TagId>>,
    pub with_attendance: bool,
}

#[derive(Debug)]
pub struct CreateEventResponse {
    pub id: crate::EventId,
    pub author: crate::CreateUserResponse,
    pub image_urls: Vec<Url>,
    pub title: crate::EventTitle,
    pub description: crate::EventDescription,
    pub tags: Vec<crate::CreateTagResponse>,
    pub with_attendance: bool,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

#[derive(Error, Debug)]
#[error(transparent)]
pub enum CreateEventError {
    Db(#[from] eyre::Error),
}

#[nutype(derive(Debug, PartialEq, Eq, Hash))]
pub struct EventId(Uuid);

#[nutype(new_unchecked, derive(Debug, PartialEq, Eq, AsRef, Hash))]
pub struct EventTitle(String);

#[nutype(new_unchecked, derive(Debug, PartialEq, Eq, AsRef, Hash))]
pub struct EventDescription(String);
