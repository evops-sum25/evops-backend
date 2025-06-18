use diesel::result::DatabaseErrorKind;
use diesel::{ExpressionMethods as _, Insertable, QueryDsl as _, SelectableHelper as _};
use diesel_async::scoped_futures::ScopedFutureExt as _;
use diesel_async::{AsyncConnection as _, RunQueryDsl as _};
use uuid::Uuid;

#[derive(Insertable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewTag<'a> {
    id: Uuid,
    name: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::tag_aliases)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewTagAlias<'a> {
    tag_id: Uuid,
    alias: &'a str,
}

impl crate::Database {
    pub async fn create_tag(
        &mut self,
        form: evops_models::NewTagForm,
    ) -> Result<evops_models::Tag, evops_models::CreateTagError> {
        self.conn
            .transaction(|conn| {
                async move {
                    let tag_id = Uuid::now_v7();

                    let insert_tag_result = {
                        diesel::insert_into(crate::schema::tags::table)
                            .values(NewTag {
                                id: tag_id,
                                name: form.name.as_ref(),
                            })
                            .execute(conn)
                            .await
                    };
                    if let Err(diesel::result::Error::DatabaseError(
                        DatabaseErrorKind::UniqueViolation,
                        info,
                    )) = insert_tag_result
                    {
                        return Err(evops_models::CreateTagError::AlreadyExists({
                            info.message().to_owned()
                        }));
                    }

                    let aliases = form.aliases.unwrap_or_default();
                    diesel::insert_into(crate::schema::tag_aliases::table)
                        .values({
                            aliases
                                .iter()
                                .map(|a| NewTagAlias {
                                    tag_id,
                                    alias: a.as_ref(),
                                })
                                .collect::<Vec<_>>()
                        })
                        .execute(conn)
                        .await?;

                    Ok(evops_models::Tag {
                        id: evops_models::TagId::new(tag_id),
                        name: form.name,
                        aliases,
                    })
                }
                .scope_boxed()
            })
            .await
    }

    pub async fn find_tag(
        &mut self,
        id: evops_models::TagId,
    ) -> Result<evops_models::Tag, evops_models::FindTagError> {
        self.conn
            .transaction(|conn| {
                async move {
                    let find_tag_result: Result<crate::models::Tag, diesel::result::Error> = {
                        crate::schema::tags::table
                            .find(id.into_inner())
                            .select(crate::models::Tag::as_select())
                            .get_result(conn)
                            .await
                    };

                    let aliases = {
                        let find_result = {
                            crate::schema::tag_aliases::table
                                .filter(crate::schema::tag_aliases::tag_id.eq(id.into_inner()))
                                .select(crate::models::TagAlias::as_select())
                                .load(conn)
                                .await
                        };
                        match find_result {
                            Ok(aliases) => aliases
                                .into_iter()
                                .map(|a| unsafe { evops_models::TagAlias::new_unchecked(a.alias) })
                                .collect(),
                            Err(e) => return Err(e.into()),
                        }
                    };

                    match find_tag_result {
                        Ok(tag) => Ok(evops_models::Tag {
                            id: evops_models::TagId::new(tag.id),
                            name: unsafe { evops_models::TagName::new_unchecked(tag.name) },
                            aliases,
                        }),
                        Err(e) => Err(match e {
                            diesel::result::Error::NotFound => {
                                evops_models::FindTagError::NotFound(id)
                            }
                            _ => e.into(),
                        }),
                    }
                }
                .scope_boxed()
            })
            .await
    }
}
