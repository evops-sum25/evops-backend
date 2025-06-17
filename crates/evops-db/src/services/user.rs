use diesel::{Insertable, SelectableHelper as _};
use diesel_async::RunQueryDsl as _;
use evops_types::{CreateUserError, User, UserForm};
use url::Url;
use uuid::Uuid;

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewUser<'a> {
    id: Uuid,
    name: &'a str,
    profile_picture_url: Option<&'a str>,
}

impl crate::Database {
    pub async fn create_user(&mut self, form: UserForm) -> Result<User, CreateUserError> {
        let user_id = Uuid::now_v7();

        diesel::insert_into(crate::schema::users::table)
            .values(NewUser {
                id: user_id,
                name: form.name.as_ref(),
                profile_picture_url: form.profile_picture_url.as_ref().map(Url::as_str),
            })
            .returning(crate::models::User::as_returning())
            .execute(&mut self.conn)
            .await
            .map_err(|e| CreateUserError::Db(e.into()))?;

        Ok(User {
            id: evops_types::UserId::new(user_id),
            name: form.name,
            profile_picture_url: form.profile_picture_url,
        })
    }
}
