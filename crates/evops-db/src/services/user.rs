use diesel::{Insertable, QueryDsl as _, SelectableHelper as _};
use diesel_async::scoped_futures::ScopedFutureExt as _;
use diesel_async::{AsyncConnection as _, RunQueryDsl as _};
use url::Url;
use uuid::Uuid;

use crate::models;
use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
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
        id: evops_models::UserId,
    ) -> Result<evops_models::User, evops_models::FindUserError> {
        self.conn
            .transaction(|conn| {
                async move {
                    let user: models::User = {
                        schema::users::table
                            .find(id.into_inner())
                            .select(models::User::as_select())
                            .get_result(conn)
                            .await
                            .map_err(|e| match e {
                                diesel::result::Error::NotFound => {
                                    evops_models::FindUserError::NotFound(id)
                                }
                                _ => e.into(),
                            })?
                    };

                    Ok(evops_models::User {
                        id,
                        name: unsafe { evops_models::UserName::new_unchecked(user.name) },
                        profile_picture_url: user.profile_picture_url.map(|s| s.parse().unwrap()),
                    })
                }
                .scope_boxed()
            })
            .await
    }

    pub async fn list_users(
        &mut self,
    ) -> Result<Vec<evops_models::User>, evops_models::ListUsersError> {
        self.conn
            .transaction(|conn| {
                async move {
                    let raw_results: Vec<models::User> = schema::users::table
                        .select(models::User::as_select())
                        .get_results(conn)
                        .await?;

                    Ok(raw_results
                        .into_iter()
                        .map(|u| evops_models::User {
                            id: evops_models::UserId::new(u.id),
                            name: unsafe { evops_models::UserName::new_unchecked(u.name) },
                            profile_picture_url: u.profile_picture_url.map(|s| s.parse().unwrap()),
                        })
                        .collect::<Vec<_>>())
                }
                .scope_boxed()
            })
            .await
    }

    pub async fn create_user(
        &mut self,
        form: evops_models::NewUserForm,
    ) -> Result<evops_models::User, evops_models::CreateUserError> {
        self.conn
            .transaction(|conn| {
                async move {
                    let user_id = Uuid::now_v7();

                    diesel::insert_into(schema::users::table)
                        .values(NewUser {
                            id: user_id,
                            name: form.name.as_ref(),
                            profile_picture_url: form.profile_picture_url.as_ref().map(Url::as_str),
                        })
                        .returning(models::User::as_returning())
                        .execute(conn)
                        .await?;

                    Ok(evops_models::User {
                        id: evops_models::UserId::new(user_id),
                        name: form.name,
                        profile_picture_url: form.profile_picture_url,
                    })
                }
                .scope_boxed()
            })
            .await
    }
}
