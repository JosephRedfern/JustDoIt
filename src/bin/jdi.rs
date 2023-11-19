use justdoit::{complete_task, create_task, models::*};

use diesel::prelude::*;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Done { id: i32 },
    Add { task: String },
}

fn main() {
    let connection = &mut justdoit::establish_connection();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { task }) => {
            create_task(connection, task);
            show_outstanding(connection);
        }
        Some(Commands::Done { id }) => {
            complete_task(connection, *id);
            show_outstanding(connection);
        }
        None => {
            show_outstanding(connection);
        }
    }
}

fn show_outstanding(connection: &mut SqliteConnection) {
    use justdoit::schema::tasks::dsl::*;

    let results = tasks
        .filter(completed.is_null())
        .select(Task::as_select())
        .load(connection)
        .expect("Error loading posts");

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
