pub mod models;
pub mod schema;
pub mod tasks;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dirs::data_dir;
use dotenvy::dotenv;
use std::env;

pub const SQLITE_MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/sqlite/");

const DEFAULT_FILENAME: &str = "jdi.db";

fn find_connection_string() -> String {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL");

    match database_url {
        Ok(url) => return url,
        Err(_) => {
            match data_dir() {
                Some(path) => {
                    let path = path.join(DEFAULT_FILENAME);
                    return format!("sqlite://{path_str}", path_str = path.to_str().unwrap());
                }
                None => {
                    panic!("Couldn't resolve data directory and no DATABASE_URL environment variable set")
                }
            }
        }
    }
}

fn run_migrations(connection: &mut SqliteConnection) {
    log::debug!("Ensuring latest migrations are applied");
    connection
        .run_pending_migrations(SQLITE_MIGRATIONS)
        .unwrap();
}

pub fn establish_connection() -> SqliteConnection {
    let database_url = find_connection_string();

    log::debug!("Using database at {database_url}");

    let mut conn = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    run_migrations(&mut conn);

    return conn;
}

use self::models::NewTask;

pub fn create_task(conn: &mut SqliteConnection, body: &str) {
    use self::schema::tasks;

    let new_post = NewTask { body };

    match diesel::insert_into(tasks::table)
        .values(&new_post)
        .execute(conn)
    {
        Ok(_) => println!("Task created"),
        Err(e) => println!("Error creating task: {}", e),
    }
}

pub fn complete_task(conn: &mut SqliteConnection, task_id: i32) -> bool {
    use crate::schema::tasks::dsl::*;
    use diesel::dsl::now;

    let res = diesel::update(tasks)
        .filter(id.eq(task_id))
        .set(completed.eq(now))
        .execute(conn);

    match res {
        Err(_) => false,
        Ok(_) => true,
    }
}
