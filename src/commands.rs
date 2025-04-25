mod generate;
mod show;
mod solve;

use anyhow::Result;
use clap::Subcommand;

#[derive(clap::ValueEnum, Default, Clone, Debug)]
pub enum BoardBackend {
    Grid,
    #[default]
    BitField,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Generate a new board
    Generate(generate::GenerateArgs),
    /// Solve a board
    Solve(solve::SolveArgs),
    /// Show a board
    Show(show::ShowArgs),
}

pub(crate) fn run_command(command: &Commands) -> Result<()> {
    match command {
        Commands::Generate(args) => generate::cmd_generate(args),
        Commands::Solve(args) => solve::cmd_solve(args),
        Commands::Show(args) => show::cmd_show(args),
    }
}
