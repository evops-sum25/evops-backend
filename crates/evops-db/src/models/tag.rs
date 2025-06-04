use diesel::{Identifiable, Insertable, Queryable, Selectable};
use diesel_derive_newtype::DieselNewType;

pub mod prelude {
    pub use super::{Tag, TagId};
}

#[derive(Queryable, Selectable, Identifiable, Insertable)]
#[diesel(table_name=crate::schema::tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tag {
    pub id: TagId,
    pub name: String,
}

#[derive(DieselNewType, Debug, Hash, PartialEq, Eq)]
pub struct TagId(i64);
