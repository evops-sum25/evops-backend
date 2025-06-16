use diesel::Insertable;
use diesel::result::DatabaseErrorKind;
use diesel_async::RunQueryDsl as _;
use uuid::Uuid;

impl crate::Database {
    pub async fn create_tag(
        &mut self,
        request: evops_types::CreateTagRequest,
    ) -> Result<evops_types::CreateTagResponse, crate::errors::CreateTagError> {
        let tag_id = Uuid::now_v7();

        #[derive(Insertable)]
        #[diesel(table_name = crate::schema::tags)]
        #[diesel(check_for_backend(diesel::pg::Pg))]
        struct NewTag<'a> {
            id: &'a Uuid,
            name: &'a str,
        }
        let insert_result = diesel::insert_into(crate::schema::tags::table)
            .values(NewTag {
                id: &tag_id,
                name: request.name.as_ref(),
            })
            .execute(&mut self.conn)
            .await;

        if let Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info)) =
            insert_result
        {
            return Err(crate::errors::CreateTagError::Duplicate({
                if let Some(details) = info.details() {
                    format!("{}\n\n{details}", info.message())
                } else {
                    info.message().to_owned()
                }
            }));
        }

        let aliases = request.aliases.unwrap_or_default();

        #[derive(Insertable)]
        #[diesel(table_name = crate::schema::tag_aliases)]
        #[diesel(check_for_backend(diesel::pg::Pg))]
        struct NewTagAlias<'a> {
            tag_id: &'a Uuid,
            alias: &'a str,
        }
        diesel::insert_into(crate::schema::tag_aliases::table)
            .values({
                aliases
                    .iter()
                    .map(|a| NewTagAlias {
                        tag_id: &tag_id,
                        alias: a.as_ref(),
                    })
                    .collect::<Vec<_>>()
            })
            .execute(&mut self.conn)
            .await?;

        Ok(evops_types::CreateTagResponse {
            id: evops_types::TagId::new(tag_id),
            name: request.name,
            aliases,
        })
    }
}
