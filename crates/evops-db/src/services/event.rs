use chrono::{DateTime, Utc};
use diesel::Insertable;
use diesel_async::RunQueryDsl as _;
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
    pub async fn create_event(
        &mut self,
        form: evops_types::NewEventForm,
    ) -> Result<evops_types::Event, evops_types::CreateEventError> {
        let author = match self.find_user(form.author_id).await {
            Ok(author) => author,
            Err(e) => {
                return Err(match e {
                    evops_types::FindUserError::NotFound(e) => {
                        evops_types::CreateEventError::AuthorNotFound(e)
                    }
                    evops_types::FindUserError::Db(e) => evops_types::CreateEventError::Db(e),
                });
            }
        };

        let tags = {
            let tag_ids = form.tag_ids.unwrap_or_default();
            let mut buffer = Vec::with_capacity(tag_ids.len());

            for id in tag_ids {
                match self.find_tag(id).await {
                    Ok(tag) => {
                        buffer.push(tag);
                    }
                    Err(e) => {
                        return Err(match e {
                            evops_types::FindTagError::NotFound(e) => {
                                evops_types::CreateEventError::TagNotFound(e)
                            }
                            evops_types::FindTagError::Db(e) => {
                                evops_types::CreateEventError::Db(e)
                            }
                        });
                    }
                }
            }

            buffer
        };

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
            .map_err(|e| evops_types::CreateEventError::Db(e.into()))?;

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
            .map_err(|e| evops_types::CreateEventError::Db(e.into()))?;

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
            .map_err(|e| evops_types::CreateEventError::Db(e.into()))?;

        diesel::insert_into(crate::schema::event_tags::table)
            .values({
                tags.iter()
                    .map(|t| NewEventTag {
                        event_id,
                        tag_id: t.id.into_inner(),
                    })
                    .collect::<Vec<_>>()
            })
            .execute(&mut self.conn)
            .await
            .map_err(|e| evops_types::CreateEventError::Db(e.into()))?;

        Ok(evops_types::Event {
            id: evops_types::EventId::new(event_id),
            author,
            image_urls,
            title: form.title,
            description: form.description,
            tags,
            with_attendance: form.with_attendance,
            created_at: now,
            modified_at: now,
        })
    }
}
