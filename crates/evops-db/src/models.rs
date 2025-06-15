use chrono::{DateTime, Utc};
use diesel::{Identifiable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::event_tags)]
#[diesel(primary_key(event_id, tag_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EventTag {
    pub event_id: Uuid,
    pub tag_id: Uuid,
}
