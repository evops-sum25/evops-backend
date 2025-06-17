use diesel::{Insertable, QueryDsl as _, SelectableHelper as _};
use diesel_async::scoped_futures::ScopedFutureExt as _;
use diesel_async::{AsyncConnection as _, RunQueryDsl as _};
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

#[allow(clippy::missing_panics_doc)]
impl crate::Database {
    pub async fn find_user(
        &mut self,
        id: evops_types::UserId,
    ) -> Result<evops_types::User, evops_types::FindUserError> {
        self.conn
            .transaction(|conn| {
                async move {
                    let user: crate::models::User = {
                        crate::schema::users::table
                            .find(id.into_inner())
                            .select(crate::models::User::as_select())
                            .get_result(conn)
                            .await
                            .map_err(|e| match e {
                                diesel::result::Error::NotFound => {
                                    evops_types::FindUserError::NotFound(id)
                                }
                                _ => e.into(),
                            })?
                    };

                    Ok(evops_types::User {
                        id,
                        name: unsafe { evops_types::UserName::new_unchecked(user.name) },
                        profile_picture_url: user.profile_picture_url.map(|s| s.parse().unwrap()),
                    })
                }
                .scope_boxed()
            })
            .await
    }

    pub async fn create_user(
        &mut self,
        form: evops_types::NewUserForm,
    ) -> Result<evops_types::User, evops_types::CreateUserError> {
        self.conn
            .transaction(|conn| {
                async move {
                    let user_id = Uuid::now_v7();

                    diesel::insert_into(crate::schema::users::table)
                        .values(NewUser {
                            id: user_id,
                            name: form.name.as_ref(),
                            profile_picture_url: form.profile_picture_url.as_ref().map(Url::as_str),
                        })
                        .returning(crate::models::User::as_returning())
                        .execute(conn)
                        .await?;

                    Ok(evops_types::User {
                        id: evops_types::UserId::new(user_id),
                        name: form.name,
                        profile_picture_url: form.profile_picture_url,
                    })
                }
                .scope_boxed()
            })
            .await
    }
}
