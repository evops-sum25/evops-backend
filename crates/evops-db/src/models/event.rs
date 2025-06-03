use chrono::{DateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use diesel_derive_newtype::DieselNewType;

use crate::models::location::LocationId;
use crate::models::user::UserId;

pub mod prelude {
    pub use super::{Event, EventId};
}

#[derive(Queryable, Selectable, Identifiable, Insertable)]
#[diesel(table_name=crate::schema::events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Event {
    pub id: EventId,
    pub title: String,
    pub description: String,
    pub author: UserId,
    pub location_id: Option<LocationId>,
    pub created_at: DateTime<Utc>,
}

#[derive(DieselNewType, Debug, Hash, PartialEq, Eq)]
pub struct EventId(i64);
