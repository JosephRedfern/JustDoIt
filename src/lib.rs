pub mod models;
pub mod schema;
pub mod tasks;

use chrono::expect;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::NewTask;

pub fn create_task(conn: &mut SqliteConnection, task: &str) {
    use self::schema::tasks;

    let new_post = NewTask { task };

    match diesel::insert_into(tasks::table)
        .values(&new_post)
        .execute(conn)
    {
        Ok(_) => println!("Task created"),
        Err(e) => println!("Error creating task: {}", e),
    }
}
