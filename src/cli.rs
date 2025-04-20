mod generate;

use crate::cli::generate::generate;
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
    /// Generate a new grid
    Generate {
        seed: Option<u64>,
        #[arg(short, long, default_value_t = 200)]
        max_iterations: usize,
        #[arg(short, long, default_value_t = 9)]
        size: usize,
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
            } => generate(max_iterations, seed, size),
        }
    }
}
