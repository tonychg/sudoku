use std::path::PathBuf;

use super::BoardBackend;
use crate::board::BitFieldBoard;
use crate::board::BoardGenerator;
use crate::board::GridBoard;
use crate::board::to_pretty_grid;
use anyhow::Result;

#[derive(clap::Args, Clone, Debug)]
pub(crate) struct GenerateArgs {
    /// Specify seed of generated board
    seed: Option<u64>,
    /// Maximum iterations of complete board recursion
    #[arg(short, long, default_value_t = 2000)]
    max_iterations: usize,
    /// Size of board
    #[arg(short, long, default_value_t = 9)]
    size: usize,
    /// Define starting numbers count
    #[arg(short = 'n', long, default_value_t = 40)]
    starting_numbers: usize,
    /// Use raw format to print the generated board
    #[arg(short = 'r', long, default_value_t = false)]
    raw: bool,
    /// Select board storage backend
    #[arg(short = 'b', long, value_enum, default_value_t = BoardBackend::BitField)]
    backend: BoardBackend,
    /// Path to destination directory
    #[arg(short = 'd', long)]
    destination: Option<PathBuf>,
}

#[tracing::instrument]
pub(crate) fn cmd_generate(args: &GenerateArgs) -> Result<()> {
    let generator = BoardGenerator::new(
        args.size,
        args.seed,
        args.starting_numbers,
        args.max_iterations,
    );
    match args.backend {
        BoardBackend::BitField => {
            let board = generator.generate::<BitFieldBoard>()?;
            let playable = generator.make_playable(&board);
            if args.raw {
                println!("{}", board);
                println!("{}", playable);
            } else {
                println!("{}", to_pretty_grid(&board));
                println!("{}", to_pretty_grid(&playable));
            }
        }
        BoardBackend::Grid => {
            let board = generator.generate::<GridBoard>()?;
            let playable = generator.make_playable(&board);
            if args.raw {
                println!("{}", board);
                println!("{}", playable);
            } else {
                println!("{}", to_pretty_grid(&board));
                println!("{}", to_pretty_grid(&playable));
            }
        }
    };

    Ok(())
}
