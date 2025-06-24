use diesel::{Insertable, QueryDsl as _, SelectableHelper as _};
use diesel_async::scoped_futures::ScopedFutureExt as _;
use diesel_async::{AsyncConnection as _, RunQueryDsl as _};
use uuid::Uuid;

use evops_models::{ApiError, ApiResult};

use crate::models;
use crate::schema;

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewUser<'a> {
    id: Uuid,
    name: &'a str,
}

#[allow(clippy::missing_panics_doc)]
impl crate::Database {
    pub async fn find_user(&mut self, id: evops_models::UserId) -> ApiResult<evops_models::User> {
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
                                    ApiError::NotFound(format!("No user with ID {id} found."))
                                }
                                _ => e.into(),
                            })?
                    };

                    Ok(evops_models::User {
                        id,
                        name: unsafe { evops_models::UserName::new_unchecked(user.name) },
                    })
                }
                .scope_boxed()
            })
            .await
    }

    pub async fn list_users(&mut self) -> ApiResult<Vec<evops_models::User>> {
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
    ) -> ApiResult<evops_models::User> {
        self.conn
            .transaction(|conn| {
                async move {
                    let user_id = Uuid::now_v7();

                    diesel::insert_into(schema::users::table)
                        .values(NewUser {
                            id: user_id,
                            name: form.name.as_ref(),
                        })
                        .returning(models::User::as_returning())
                        .execute(conn)
                        .await?;

                    Ok(evops_models::User {
                        id: evops_models::UserId::new(user_id),
                        name: form.name,
                    })
                }
                .scope_boxed()
            })
            .await
    }
}
