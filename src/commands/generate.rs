use crate::grid;
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

pub(crate) fn cmd_generate(args: &GenerateArgs) -> Result<()> {
    let seed = match args.seed {
        Some(seed) => seed,
        None => rng::generate_seed(),
    };
    let grid = grid::generate(args.size, seed, args.max_iterations);
    let playable = grid::make_playable(&grid, args.starting_numbers, seed);

    println!("seed: {}", seed);

    if args.raw {
        grid::print_raw(&grid);
        println!();
        grid::print_raw(&playable);
    } else {
        grid::print_pretty(&grid);
        println!();
        grid::print_pretty(&playable);
    }
    Ok(())
}
