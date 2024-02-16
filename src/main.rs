use clap::{Parser, Subcommand};
use std::io::{self, Read};
use tui::choose;
use urls::find_urls;

mod tui;
mod urls;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// View all notes
    List {},
    Choose {},
}

fn list() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        eprintln!("Error reading from stdin");
    }

    for url in find_urls(&input) {
        println!("{}", url);
    }
}

fn cho() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        eprintln!("Error reading from stdin");
    }
    let urls = find_urls(&input);
    let _ = choose(urls);
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List {} => {
            list();
        }
        Commands::Choose {} => {
            cho();
        }
    };
}
