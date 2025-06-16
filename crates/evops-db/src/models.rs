use chrono::{DateTime, Utc};
use diesel::{Identifiable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub profile_picture_url: Option<String>,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::images)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Image {
    pub id: Uuid,
    pub url: String,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::tag_aliases)]
#[diesel(primary_key(tag_id, alias))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TagAlias {
    pub tag_id: Uuid,
    pub alias: String,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub author_id: Uuid,
    pub with_attendance: bool,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::event_images)]
#[diesel(primary_key(event_id, image_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EventImage {
    pub event_id: Uuid,
    pub image_id: Uuid,
}

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::event_tags)]
#[diesel(primary_key(event_id, tag_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EventTag {
    pub event_id: Uuid,
    pub tag_id: Uuid,
}
