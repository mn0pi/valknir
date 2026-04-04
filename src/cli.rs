use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "valknir")]
#[command(about = "Explainable heap analysis for C programs")]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Analyse { file: String },
}
