mod generate;
mod solve;

use crate::cli::generate::generate_command;
use crate::cli::solve::solve_command;
use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new board
    Generate {
        seed: Option<u64>,
        #[arg(short, long, default_value_t = 200)]
        max_iterations: usize,
        #[arg(short, long, default_value_t = 9)]
        size: usize,
        #[arg(long, default_value_t = 40)]
        starting_numbers: usize,
        #[arg(short, long, default_value_t = false)]
        raw: bool,
    },
    /// Solve a board
    Solve {
        #[arg(short, long, default_value_t = false)]
        stdin: bool,
    },
}

pub struct CliRunner;

impl CliRunner {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Result<()> {
        match Cli::parse().command {
            Commands::Generate {
                max_iterations,
                seed,
                size,
                starting_numbers,
                raw,
            } => generate_command(max_iterations, seed, size, starting_numbers, raw),
            Commands::Solve { stdin } => solve_command(stdin),
        }
    }
}
