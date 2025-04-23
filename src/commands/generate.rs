use crate::board;
use crate::rng;
use anyhow::Result;

#[derive(clap::Args, Clone, Debug)]
pub(crate) struct GenerateArgs {
    /// Specify seed of generated board
    seed: Option<u64>,
    /// Maximum iterations of complete board recursion
    #[arg(short, long, default_value_t = 200)]
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
}

#[tracing::instrument]
pub(crate) fn cmd_generate(args: &GenerateArgs) -> Result<()> {
    let seed = args.seed.unwrap_or(rng::generate_seed());
    let grid = board::generate(args.size, seed, args.max_iterations);
    let playable = board::make_playable(&grid, args.starting_numbers, seed);

    tracing::info!(seed);

    if args.raw {
        board::print_raw(&grid);
        println!();
        board::print_raw(&playable);
    } else {
        board::print_pretty(&grid);
        println!();
        board::print_pretty(&playable);
    }
    Ok(())
}
