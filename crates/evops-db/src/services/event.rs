use std::collections::HashMap;

use chrono::{DateTime, Utc};
use diesel::{
    BelongingToDsl as _, ExpressionMethods as _, GroupedBy as _, Insertable, QueryDsl as _,
    SelectableHelper as _,
};
use diesel_async::scoped_futures::ScopedFutureExt as _;
use diesel_async::{AsyncConnection as _, RunQueryDsl as _};

use uuid::Uuid;

use evops_models::{ApiError, ApiResult, PgLimit};

use tracing::{debug};
use itertools::Itertools;

use crate::models;
use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::images)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewImage<'a> {
    id: Uuid,
    url: &'a str,
    event_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = schema::events)]
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
#[diesel(table_name = schema::events_tags)]
#[diesel(primary_key(event_id, tag_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewEventTag {
    event_id: Uuid,
    tag_id: Uuid,
}

impl crate::Database {
    #[allow(clippy::missing_panics_doc)]
    pub async fn find_event(
        &mut self,
        id: evops_models::EventId,
    ) -> ApiResult<evops_models::Event> {
        self.conn
            .transaction(|conn| {
                async move {
                    let event: models::Event = {
                        schema::events::table
                            .find(id.into_inner())
                            .select(models::Event::as_select())
                            .get_result(conn)
                            .await
                            .map_err(|e| match e {
                                diesel::result::Error::NotFound => {
                                    ApiError::NotFound(format!("No event with ID {id} found."))
                                }
                                e => e.into(),
                            })?
                    };

                    let author: models::User = {
                        schema::users::table
                            .find(event.author_id)
                            .select(models::User::as_select())
                            .get_result(conn)
                            .await?
                    };

                    let images: Vec<models::Image> =
                        models::Image::belonging_to(&event).load(conn).await?;

                    let tags: Vec<models::Tag> = {
                        models::EventTag::belonging_to(&event)
                            .inner_join(schema::tags::table)
                            .select(models::Tag::as_select())
                            .load(conn)
                            .await?
                    };

                    let aliases: Vec<models::TagAlias> = {
                        models::TagAlias::belonging_to(&tags)
                            .select(models::TagAlias::as_select())
                            .load(conn)
                            .await?
                    };

                    let tags_with_aliases: Vec<(models::Tag, Vec<models::TagAlias>)> = {
                        aliases
                            .grouped_by(&tags)
                            .into_iter()
                            .zip(tags)
                            .map(|(a, t)| (t, a))
                            .collect()
                    };

                    Ok(evops_models::Event {
                        id,
                        author: evops_models::User {
                            id: evops_models::UserId::new(author.id),
                            name: unsafe { evops_models::UserName::new_unchecked(author.name) },
                        },
                        image_urls: {
                            images
                                .into_iter()
                                .map(|img| img.url.parse().unwrap())
                                .collect()
                        },
                        title: unsafe { evops_models::EventTitle::new_unchecked(event.title) },
                        description: unsafe {
                            evops_models::EventDescription::new_unchecked(event.description)
                        },
                        tags: {
                            tags_with_aliases
                                .into_iter()
                                .map(|(tag, aliases)| evops_models::Tag {
                                    id: evops_models::TagId::new(tag.id),
                                    name: unsafe { evops_models::TagName::new_unchecked(tag.name) },
                                    aliases: {
                                        aliases
                                            .into_iter()
                                            .map(|a| unsafe {
                                                evops_models::TagAlias::new_unchecked(a.alias)
                                            })
                                            .collect()
                                    },
                                })
                                .collect()
                        },
                        with_attendance: event.with_attendance,
                        created_at: event.created_at,
                        modified_at: event.modified_at,
                    })
                }
                .scope_boxed()
            })
            .await
    }

    #[allow(clippy::missing_panics_doc, clippy::too_many_lines)]
    pub async fn list_events(
        &mut self,
        last_id: Option<evops_models::EventId>,
        limit: Option<PgLimit>,
    ) -> ApiResult<(Vec<evops_models::Event>, Option<evops_models::EventId>)> {
        debug!("new list event request: last_id {:?}, limit {:?}", last_id, limit);
        self.conn
            .transaction(|conn| {
                async move {
                    let event_ids: Vec<Uuid> = {
                        let mut query = schema::events::table
                            .select(schema::events::id) 
                            .into_boxed(); // Runtime query

                        if let Some(id) = last_id {
                            query = query.filter(schema::events::id.gt(id.into_inner()));
                        }

                        query = query.order(schema::events::id.asc());

                        if let Some(lim) = limit {
                            
                            query = query.limit(lim.into());
                            
                        }
                        query
                            .load(conn)
                            .await?
                    };
                    
                    if event_ids.is_empty() {
                        return Ok((Vec::new(), None)); // Nothing to do
                    }

                    let events_with_authors: Vec<(models::Event, models::User)> = {
                        schema::events::table
                            .inner_join(schema::users::table)
                            .filter(schema::events::id.eq_any(&event_ids))
                            .select((models::Event::as_select(), models::User::as_select()))
                            .load(conn)
                            .await?
                    };
                    let images = {
                        schema::images::table
                            .filter(schema::images::event_id.eq_any(&event_ids))
                            .select(models::Image::as_select())
                            .load(conn)
                            .await?
                            .into_iter()
                            .into_group_map_by(|img| img.event_id)
                    };

                    let tags: HashMap<Uuid, HashMap<models::Tag, Option<Vec<models::TagAlias>>>> = {
                        let event_tags:Vec<(Uuid, models::Tag)> = schema::events_tags::table
                            .filter(schema::events_tags::event_id.eq_any(&event_ids))
                            .inner_join(schema::tags::table)
                            .select((schema::events_tags::event_id, models::Tag::as_select()))
                            .load::<(Uuid, models::Tag)>(conn)
                            .await?;

                        let tag_ids: Vec<Uuid> = event_tags.iter().map(|(_, tag)| tag.id).collect();
                        let tag_aliases = {
                            schema::tags_aliases::table
                                .filter(schema::tags_aliases::tag_id.eq_any(tag_ids))
                                .select(models::TagAlias::as_select())
                                .load(conn)
                                .await?
                                .into_iter()
                                .into_group_map_by(|alias| alias.tag_id)
                        };
                        event_tags
                            .into_iter()
                            .fold(HashMap::new(), |mut outer_map, (event_id, tag)| {                                
                                outer_map
                                    .entry(event_id)
                                    .or_default()
                                    .insert(tag.clone(), tag_aliases.get(&tag.id).cloned());
                            
                                outer_map
                            })
                    };

                    let events : Vec<evops_models::Event> = {
                        events_with_authors
                        .into_iter()
                        .map(|(event, author)| evops_models::Event {
                            id: evops_models::EventId::new(event.id),
                            author: evops_models::User {
                                id: evops_models::UserId::new(author.id),
                                name: unsafe { evops_models::UserName::new_unchecked(author.name) },
                            },
                            image_urls: {
                                images
                                    .get(&event.id)
                                    .unwrap_or(&Vec::new())
                                    .into_iter()
                                    .map(|img| url::Url::parse(img.url.as_str()).unwrap())
                                    .collect()
                            },
                            title: unsafe { evops_models::EventTitle::new_unchecked(event.title) },
                            description: unsafe {
                                evops_models::EventDescription::new_unchecked(event.description)
                            },
                            tags: {
                                tags
                                    .get(&event.id)
                                    .unwrap_or(&HashMap::new())
                                    .into_iter()
                                    .map(|t: (&models::Tag, &Option<Vec<models::TagAlias>>)| evops_models::Tag {
                                        id: evops_models::TagId::new(t.0.id),
                                        name: unsafe { evops_models::TagName::new_unchecked(t.0.name.clone()) },
                                        aliases: {
                                            match t.1 {
                                                Some(aliases) => {
                                                    aliases
                                                        .into_iter()
                                                        .map(|alias: &models::TagAlias| 
                                                            unsafe { evops_models::TagAlias::new_unchecked(alias.alias.clone()) })
                                                        .collect()
                                                },
                                                _ => Vec::new(),
                                            }
                                        },
                                    })
                                    .collect()
                            },
                            with_attendance: event.with_attendance,
                            created_at: event.created_at,
                            modified_at: event.modified_at,
                        })
                        .collect()
                    };
                    debug!("result ready");
                    Ok((
                        events,
                        event_ids.last().map(|id| evops_models::EventId::new(*id)),
                    ))
                }
                .scope_boxed()
            })
            .await
    }

    #[allow(clippy::missing_panics_doc, clippy::too_many_lines)]
    pub async fn create_event(
        &mut self,
        form: evops_models::NewEventForm,
    ) -> ApiResult<evops_models::Event> {
        self.conn
            .transaction(|conn| {
                async move {
                    let author = {
                        let user: models::User = {
                            schema::users::table
                                .find(form.author_id.into_inner())
                                .select(models::User::as_select())
                                .get_result(conn)
                                .await
                                .map_err(|e| match e {
                                    diesel::result::Error::NotFound => ApiError::InvalidArgument(
                                        format!("No author with ID {} found", form.author_id),
                                    ),
                                    _ => e.into(),
                                })?
                        };

                        evops_models::User {
                            id: form.author_id,
                            name: unsafe { evops_models::UserName::new_unchecked(user.name) },
                        }
                    };

                    let tags = {
                        let tag_ids = form.tag_ids.unwrap_or_default();
                        let mut buffer = Vec::with_capacity(tag_ids.len());

                        for id in tag_ids {
                            let find_tag_result: Result<models::Tag, diesel::result::Error> = {
                                schema::tags::table
                                    .find(id.into_inner())
                                    .select(models::Tag::as_select())
                                    .get_result(conn)
                                    .await
                            };

                            let aliases = {
                                let find_result = {
                                    schema::tags_aliases::table
                                        .filter(schema::tags_aliases::tag_id.eq(id.into_inner()))
                                        .select(models::TagAlias::as_select())
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
                                    name: unsafe { evops_models::TagName::new_unchecked(tag.name) },
                                    aliases,
                                }),
                                Err(e) => {
                                    return Err(match e {
                                        diesel::result::Error::NotFound => {
                                            ApiError::InvalidArgument(format!(
                                                "No tag with ID {id} found.",
                                            ))
                                        }
                                        _ => e.into(),
                                    });
                                }
                            }
                        }

                        buffer
                    };

                    let now = Utc::now();
                    let event_id = Uuid::now_v7();

                    diesel::insert_into(schema::events::table)
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
                                event_id,
                            })
                            .collect::<Vec<_>>()
                    };

                    diesel::insert_into(schema::images::table)
                        .values(&new_images)
                        .execute(conn)
                        .await?;

                    diesel::insert_into(schema::events_tags::table)
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
