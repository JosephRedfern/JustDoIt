use diesel::prelude::*;

use rocket::serde::{Deserialize, Serialize};

use chrono::NaiveDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: Option<i32>,
    pub body: Option<String>,
    pub created: Option<NaiveDateTime>,
    pub completed: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::tasks)]
pub struct NewTask<'a> {
    pub body: &'a str,
}
