mod generate;
mod solve;

use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Generate a new board
    Generate(generate::GenerateArgs),
    /// Solve a board
    Solve(solve::SolveArgs),
}

pub(crate) fn run_command(command: &Commands) -> Result<()> {
    match command {
        Commands::Generate(args) => generate::cmd_generate(args),
        Commands::Solve(args) => solve::cmd_solve(args),
    }
}
