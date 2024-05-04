use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::contacts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Contact {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub active: bool,
}
