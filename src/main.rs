mod analysis;
mod cli;
mod parser;

use crate::parser::{analyse, parse};
use clap::Parser;
use cli::{Cli, Commands};
use std::fs;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyse { file } => {
            println!("[valknir] analysing: {}", file);

            let code = fs::read_to_string(&file).expect("Could not read file");

            analyse(&code);
        }
    };
}
