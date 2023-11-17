use clap::Parser;

#[derive(Parser)]
struct Cli {
    task: String,
}

fn main() {
    let args = Cli::parse();
}
