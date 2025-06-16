use diesel::{Insertable, SelectableHelper as _};
use diesel_async::RunQueryDsl as _;
use url::Url;
use uuid::Uuid;

impl crate::Database {
    pub async fn create_user(
        &mut self,
        request: evops_types::CreateUserRequest,
    ) -> Result<evops_types::CreateUserResponse, crate::errors::CreateUserError> {
        let id = Uuid::now_v7();

        #[derive(Insertable)]
        #[diesel(table_name = crate::schema::users)]
        #[diesel(check_for_backend(diesel::pg::Pg))]
        struct NewUser<'a> {
            id: &'a Uuid,
            name: &'a str,
            profile_picture_url: Option<&'a str>,
        }
        diesel::insert_into(crate::schema::users::table)
            .values(NewUser {
                id: &id,
                name: request.name.as_ref(),
                profile_picture_url: request.profile_picture_url.as_ref().map(Url::as_str),
            })
            .returning(crate::models::User::as_returning())
            .execute(&mut self.conn)
            .await?;

        Ok(evops_types::CreateUserResponse {
            id: evops_types::UserId::new(id),
            name: request.name,
            profile_picture_url: request.profile_picture_url,
        })
    }
}
