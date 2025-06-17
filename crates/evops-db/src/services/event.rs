use chrono::{DateTime, Utc};
use diesel::Insertable;
use diesel_async::RunQueryDsl;
use evops_types::{CreateEventError, Event, EventForm};
use uuid::Uuid;

#[derive(Insertable)]
#[diesel(table_name = crate::schema::images)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewImage<'a> {
    id: Uuid,
    url: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewEvent<'a> {
    id: Uuid,
    title: &'a str,
    description: &'a str,
    author_id: Uuid,
    with_attendance: bool,
    created_at: &'a DateTime<Utc>,
    modified_at: &'a DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::event_images)]
#[diesel(primary_key(event_id, image_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewEventImage {
    event_id: Uuid,
    image_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::event_tags)]
#[diesel(primary_key(event_id, tag_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewEventTag {
    event_id: Uuid,
    tag_id: Uuid,
}

impl crate::Database {
    pub async fn create_event(&mut self, form: EventForm) -> Result<Event, CreateEventError> {
        let now = Utc::now();
        let event_id = Uuid::now_v7();

        diesel::insert_into(crate::schema::events::table)
            .values(NewEvent {
                id: event_id,
                title: form.title.as_ref(),
                description: form.description.as_ref(),
                author_id: form.author_id.into_inner(),
                with_attendance: form.with_attendance,
                created_at: &now,
                modified_at: &now,
            })
            .execute(&mut self.conn)
            .await
            .map_err(|e| CreateEventError::Db(e.into()))?;

        let image_urls = form.image_urls.unwrap_or_default();
        let new_images = {
            image_urls
                .iter()
                .map(|u| NewImage {
                    id: Uuid::now_v7(),
                    url: u.as_str(),
                })
                .collect::<Vec<_>>()
        };

        diesel::insert_into(crate::schema::images::table)
            .values(&new_images)
            .execute(&mut self.conn)
            .await
            .map_err(|e| CreateEventError::Db(e.into()))?;

        diesel::insert_into(crate::schema::event_images::table)
            .values(
                new_images
                    .iter()
                    .map(|im| NewEventImage {
                        event_id,
                        image_id: im.id,
                    })
                    .collect::<Vec<_>>(),
            )
            .execute(&mut self.conn)
            .await
            .map_err(|e| CreateEventError::Db(e.into()))?;

        let tag_ids = form.tag_ids.unwrap_or_default();

        todo!();
    }
}
