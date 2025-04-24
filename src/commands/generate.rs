use crate::board;
use crate::board::solve_dfs;
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
    let board = board::GridBoard::new(args.size);
    let seed = match args.seed {
        Some(seed) => seed,
        None => rng::generate_seed(),
    };
    let result = solve_dfs(board, Some(seed), Some(1));

    if result.is_empty() {
        return Err(anyhow::anyhow!("Failed to generate board"));
    }
    let board = result[0].clone();

    if args.raw {
        println!("{}", board);
    } else {
        println!("{}", board::to_pretty_grid(board))
    }

    Ok(())
}
