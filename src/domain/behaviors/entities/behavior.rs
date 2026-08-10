use diesel::{Selectable, deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset};

use crate::schema::behaviors;
#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = behaviors)]
pub struct Behavior {
    pub id: i32,
    pub display_name: String,
    pub content: String,
    pub is_active: bool,
}
