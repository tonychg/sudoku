use crate::board::BoardBackend;
use crate::board_generator::BoardGenerator;
use crate::destination::write_board;
use anyhow::Result;
use std::path::PathBuf;

#[derive(clap::Args, Clone, Debug)]
pub(crate) struct GenerateArgs {
    /// Specify seed of generated board
    seed: Option<u64>,
    /// Maximum depth into graph for complete board generation
    #[arg(short, long, default_value_t = 2000)]
    max_depth: usize,
    /// Size of board
    #[arg(short, long, default_value_t = 9)]
    size: usize,
    /// Define starting numbers count
    #[arg(short = 'n', long, default_value_t = 26)]
    starting_numbers: usize,
    /// Use raw format to print the generated board
    #[arg(short = 'r', long, default_value_t = false)]
    raw: bool,
    /// Path to destination directory
    #[arg(short = 'd', long)]
    destination: Option<PathBuf>,
    /// Select board storage backend
    #[arg(short = 'b', long, value_enum, default_value_t = BoardBackend::Grid)]
    backend: BoardBackend,
}

#[tracing::instrument]
pub(crate) fn cmd_generate(args: &GenerateArgs) -> Result<()> {
    let generator =
        BoardGenerator::new(args.size, args.seed, args.starting_numbers, args.max_depth);
    let board = generator.generate(&args.backend)?;
    let playable = generator.make_playable(&board);
    if let Some(destination) = &args.destination {
        write_board(&destination.as_path(), &playable)?;
    } else if args.raw {
        println!("{}", board);
        println!("{}", playable);
    } else {
        println!("{}", board.seed());
        println!("{}", board.to_pretty_grid());
        println!("{}", playable.to_pretty_grid());
    }
    Ok(())
}
