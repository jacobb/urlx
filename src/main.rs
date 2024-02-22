use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Read};
use tui::choose_from_urls;
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
    Choose {
        #[arg(long, short)]
        file_path: Option<String>,
    },
}

fn list() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        eprintln!("Error reading from stdin");
    }

    for url in find_urls(&input, &true) {
        println!("{}", url);
    }
}

fn choose_from_stdin() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        eprintln!("Error reading from stdin");
    }
    let urls = find_urls(&input, &true);
    let _ = choose_from_urls(urls);
}

fn choose_from_file(file_path_str: &str) {
    let file_contents =
        fs::read_to_string(file_path_str).expect("Something went wrong reading the file");
    let urls = find_urls(&file_contents, &true);
    let _ = choose_from_urls(urls);
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List {} => {
            list();
        }
        Commands::Choose { file_path } => {
            if let Some(path) = file_path {
                return choose_from_file(path);
            }
            choose_from_stdin();
        }
    };
}
