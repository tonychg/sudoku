use std::path::PathBuf;
use std::time::Instant;

use anyhow::Result;
use sudoku::Board;

use crate::file::list_boards;

#[derive(clap::Args, Clone, Debug)]
pub(crate) struct SolveArgs {
    /// Solve boards from file
    source: Option<PathBuf>,
    /// Read board from stdin
    #[arg(short, long, default_value_t = false)]
    stdin: bool,
    /// Use raw format to print the generated board
    #[arg(short = 'r', long, default_value_t = false)]
    raw: bool,
    /// Maximum iterations of complete board recursion
    #[arg(short, long)]
    max_iterations: Option<usize>,
    /// Limit number of solutions
    #[arg(short, long)]
    limit: Option<usize>,
}

#[tracing::instrument(skip_all)]
pub fn cmd_solve(args: &SolveArgs) -> Result<()> {
    for board in list_boards(args.source.as_ref(), args.stdin)? {
        let now = Instant::now();
        dfs_solve_board(board, args.raw);
        tracing::debug!("Solved in {} ms", now.elapsed().as_millis());
    }
    Ok(())
}

fn dfs_solve_board(board: Board, raw: bool) {
    for (index, solution) in board.backtracking(false).enumerate() {
        if raw {
            println!("{},{}", index, solution);
        } else {
            println!("{}\n{}", index, solution.to_pretty_grid());
        }
    }
}
