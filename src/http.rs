use crate::establish_connection;

use crate::models::Task;
use crate::schema::tasks;
use diesel::prelude::*;
use rocket::serde::json::Json;

#[get("/tasks/<id>")]
pub fn get_task(id: i32) -> Json<Task> {
    let connection = &mut establish_connection();

    // TODO: how can I avoid all this verbose diesel::QueryDsl::filter stuff? :(
    let result: Task = diesel::QueryDsl::filter(tasks::table, tasks::id.eq(id))
        .order_by(tasks::created.asc())
        .select(Task::as_select())
        .first(connection)
        .expect("Error loading posts");

    Json(result)
}

#[get("/tasks")]
pub fn get_tasks() -> Json<Vec<Task>> {
    let connection = &mut establish_connection();

    let result = tasks::table
        .order_by(tasks::created.asc())
        .select(Task::as_select())
        .load(connection)
        .expect("Error loading posts");

    Json(result)
}
