use diesel::{Identifiable, Insertable, Queryable, Selectable};
use diesel_derive_newtype::DieselNewType;

pub mod prelude {
    pub use super::{Location, LocationId};
}

#[derive(Queryable, Selectable, Identifiable, Insertable)]
#[diesel(table_name=crate::schema::locations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Location {
    pub id: LocationId,
    pub name: String,
}

#[derive(DieselNewType, Debug, Hash, PartialEq, Eq)]
pub struct LocationId(i64);
