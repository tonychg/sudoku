use crate::board::GridBoard;
use crate::board::to_pretty_grid;
use crate::dfs::solve_dfs;
use anyhow::Result;
use std::io;

#[derive(clap::Args, Clone, Debug)]
pub(crate) struct SolveArgs {
    /// Read board from stdin
    #[arg(short, long, default_value_t = true)]
    stdin: bool,
    /// Maximum iterations of complete board recursion
    #[arg(short, long)]
    max_iterations: Option<usize>,
    /// Limit number of solutions
    #[arg(short, long)]
    limit: Option<usize>,
}

#[tracing::instrument]
pub fn cmd_solve(args: &SolveArgs) -> Result<()> {
    if args.stdin {
        for line in io::stdin().lines().map_while(Result::ok) {
            let board = GridBoard::from_str(line)?;
            let solutions = solve_dfs(board, None, args.limit, args.max_iterations);
            if solutions.is_empty() {
                tracing::info!("No solutions");
            } else {
                for solution in solutions.iter() {
                    println!("{}", to_pretty_grid(solution));
                }
            }
        }
    }
    Ok(())
}
