use diesel::{Identifiable, Insertable, Queryable, Selectable};
use diesel_derive_newtype::DieselNewType;

pub mod prelude {
    pub use super::{EmailAddress, User, UserId};
}

#[derive(Queryable, Selectable, Identifiable, Insertable)]
#[diesel(table_name=crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: UserId,
    pub email_address: EmailAddress,
}

#[derive(DieselNewType, Debug, Hash, PartialEq, Eq)]
pub struct UserId(i64);

#[derive(DieselNewType, Debug)]
pub struct EmailAddress(String);
