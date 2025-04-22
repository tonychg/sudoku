use crate::commands::Commands;
use crate::commands::run_command;
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

pub struct CliRunner;

impl CliRunner {
    pub fn init() -> Self {
        Self
    }

    pub fn run(&self) -> Result<()> {
        run_command(&Cli::parse().command)
    }
}
