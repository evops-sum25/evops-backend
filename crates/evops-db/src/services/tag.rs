use diesel::result::DatabaseErrorKind;
use diesel::{ExpressionMethods as _, Insertable, QueryDsl as _, SelectableHelper as _};
use diesel_async::RunQueryDsl as _;
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
        form: evops_types::NewTagForm,
    ) -> Result<evops_types::Tag, evops_types::CreateTagError> {
        let tag_id = Uuid::now_v7();

        let insert_tag_result = {
            diesel::insert_into(crate::schema::tags::table)
                .values(NewTag {
                    id: tag_id,
                    name: form.name.as_ref(),
                })
                .execute(&mut self.conn)
                .await
        };
        if let Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info)) =
            insert_tag_result
        {
            return Err(evops_types::CreateTagError::Duplicate({
                info.details().map_or_else(
                    || info.message().to_owned(),
                    |details| format!("{}\n\n{details}", info.message()),
                )
            }));
        }

        let aliases = form.aliases.unwrap_or_default();
        diesel::insert_into(crate::schema::tag_aliases::table)
            .values({
                aliases
                    .iter()
                    .map(|a| NewTagAlias {
                        tag_id: tag_id,
                        alias: a.as_ref(),
                    })
                    .collect::<Vec<_>>()
            })
            .execute(&mut self.conn)
            .await
            .map_err(|e| evops_types::CreateTagError::Db(e.into()))?;

        Ok(evops_types::Tag {
            id: evops_types::TagId::new(tag_id),
            name: form.name,
            aliases,
        })
    }

    pub async fn find_tag(
        &mut self,
        id: evops_types::TagId,
    ) -> Result<evops_types::Tag, evops_types::FindTagError> {
        let find_tag_result: Result<crate::models::Tag, diesel::result::Error> = {
            crate::schema::tags::table
                .find(id.into_inner())
                .select(crate::models::Tag::as_select())
                .get_result(&mut self.conn)
                .await
        };

        let aliases = {
            let find_result = {
                crate::schema::tag_aliases::table
                    .filter(crate::schema::tag_aliases::tag_id.eq(id.into_inner()))
                    .select(crate::models::TagAlias::as_select())
                    .load(&mut self.conn)
                    .await
            };
            match find_result {
                Ok(aliases) => aliases
                    .into_iter()
                    .map(|a| unsafe { evops_types::TagAlias::new_unchecked(a.alias) })
                    .collect(),
                Err(e) => return Err(evops_types::FindTagError::Db(e.into())),
            }
        };

        match find_tag_result {
            Ok(tag) => Ok(evops_types::Tag {
                id: evops_types::TagId::new(tag.id),
                name: unsafe { evops_types::TagName::new_unchecked(tag.name) },
                aliases: aliases,
            }),
            Err(e) => Err(match e {
                diesel::result::Error::NotFound => evops_types::FindTagError::NotFound(id),
                e => evops_types::FindTagError::Db(e.into()),
            }),
        }
    }
}
