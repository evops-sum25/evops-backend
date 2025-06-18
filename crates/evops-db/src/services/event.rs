use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods as _, Insertable, QueryDsl as _, SelectableHelper as _};
use diesel_async::scoped_futures::ScopedFutureExt as _;
use diesel_async::{AsyncConnection as _, RunQueryDsl as _};
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
    pub async fn find_event(
        &mut self,
        form: evops_models::EventId,
    ) -> Result<evops_models::Event, evops_models::FindEventError> {
        self.conn
            .transaction(|conn| {
                async move {
                    todo!();
                }
                .scope_boxed()
            })
            .await
    }

    pub async fn list_events(
        &mut self,
    ) -> Result<Vec<evops_models::Event>, evops_models::ListEventsError> {
        self.conn
            .transaction(|conn| {
                async move {
                    todo!();
                }
                .scope_boxed()
            })
            .await
    }

    #[allow(clippy::missing_panics_doc, clippy::too_many_lines)]
    pub async fn create_event(
        &mut self,
        form: evops_models::NewEventForm,
    ) -> Result<evops_models::Event, evops_models::CreateEventError> {
        self.conn
            .transaction(|conn| {
                async move {
                    let author = {
                        let user: crate::models::User = {
                            crate::schema::users::table
                                .find(form.author_id.into_inner())
                                .select(crate::models::User::as_select())
                                .get_result(conn)
                                .await
                                .map_err(|e| match e {
                                    diesel::result::Error::NotFound => {
                                        evops_models::CreateEventError::AuthorNotFound({
                                            form.author_id
                                        })
                                    }
                                    _ => e.into(),
                                })?
                        };

                        evops_models::User {
                            id: form.author_id,
                            name: unsafe { evops_models::UserName::new_unchecked(user.name) },
                            profile_picture_url: user
                                .profile_picture_url
                                .map(|s| s.parse().unwrap()),
                        }
                    };

                    let tags = {
                        let tag_ids = form.tag_ids.unwrap_or_default();
                        let mut buffer = Vec::with_capacity(tag_ids.len());

                        for id in tag_ids {
                            let find_tag_result: Result<
                                crate::models::Tag,
                                diesel::result::Error,
                            > = {
                                crate::schema::tags::table
                                    .find(id.into_inner())
                                    .select(crate::models::Tag::as_select())
                                    .get_result(conn)
                                    .await
                            };

                            let aliases = {
                                let find_result = {
                                    crate::schema::tag_aliases::table
                                        .filter(
                                            crate::schema::tag_aliases::tag_id
                                                .eq(id.into_inner()),
                                        )
                                        .select(crate::models::TagAlias::as_select())
                                        .load(conn)
                                        .await
                                };
                                match find_result {
                                    Ok(aliases) => aliases
                                        .into_iter()
                                        .map(|a| unsafe {
                                            evops_models::TagAlias::new_unchecked(a.alias)
                                        })
                                        .collect(),
                                    Err(e) => return Err(e.into()),
                                }
                            };

                            match find_tag_result {
                                Ok(tag) => buffer.push(evops_models::Tag {
                                    id: evops_models::TagId::new(tag.id),
                                    name: unsafe {
                                        evops_models::TagName::new_unchecked(tag.name)
                                    },
                                    aliases,
                                }),
                                Err(e) => return Err(match e {
                                    diesel::result::Error::NotFound => {
                                        evops_models::CreateEventError::TagNotFound(id)
                                    }
                                    _ => e.into(),
                                }),
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
                        .execute(conn)
                        .await?;

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
                        .execute(conn)
                        .await?;

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
                        .execute(conn)
                        .await?;

                    diesel::insert_into(crate::schema::event_tags::table)
                        .values({
                            tags.iter()
                                .map(|t| NewEventTag {
                                    event_id,
                                    tag_id: t.id.into_inner(),
                                })
                                .collect::<Vec<_>>()
                        })
                        .execute(conn)
                        .await?;

                    Ok(evops_models::Event {
                        id: evops_models::EventId::new(event_id),
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
                .scope_boxed()
            })
            .await
    }
}
