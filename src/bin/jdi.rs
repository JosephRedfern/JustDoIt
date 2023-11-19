use justdoit::{complete_task, create_task, models::*};

use clap::{Parser, Subcommand};
use diesel::prelude::*;
use std::sync::Once;

static INIT: Once = Once::new();

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Done {
        id: i32,
    },
    Add {
        task: String,
    },
    #[command(subcommand)]
    List(ListCommands),
}

#[derive(Subcommand)]
enum ListCommands {
    All,
    Complete,
    Incomplete,
}

fn setup() {
    INIT.call_once(|| env_logger::init());
}

fn main() {
    setup();

    let connection = &mut justdoit::establish_connection();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { task }) => {
            create_task(connection, task);
            list_tasks(connection, false, true);
        }
        Some(Commands::Done { id }) => {
            complete_task(connection, *id);
            list_tasks(connection, false, true);
        }
        Some(Commands::List(args)) => match args {
            ListCommands::All => {
                list_tasks(connection, true, true);
            }
            ListCommands::Complete => {
                list_tasks(connection, true, false);
            }
            ListCommands::Incomplete => {
                list_tasks(connection, false, true);
            }
        },
        None => {
            list_tasks(connection, false, true);
        }
    }
}

fn list_tasks(connection: &mut SqliteConnection, include_complete: bool, include_incomplete: bool) {
    use justdoit::schema::tasks;

    let mut query = tasks::table.into_boxed();

    if include_complete && include_incomplete {
        // nop
    } else {
        if include_complete {
            query = query.filter(tasks::completed.is_not_null());
        }

        if include_incomplete {
            query = query.filter(tasks::completed.is_null());
        }
    }

    let results: Vec<Task> = query
        .order_by(tasks::created.asc())
        .select(Task::as_select())
        .load(connection)
        .expect("Error loading posts");

    log::info!("Found {}", results.len());

    println!("Displaying {} task(s)", results.len());
    println!("---------------------\n");

    for t in results.iter() {
        println!(
            "{}. {} ({})",
            t.id.unwrap(),
            t.body.as_ref().unwrap(),
            t.created.as_ref().unwrap()
        );
    }
}
