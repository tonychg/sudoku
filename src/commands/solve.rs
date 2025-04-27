use std::path::PathBuf;

use anyhow::Result;

use crate::board::Board;
use crate::board::BoardBackend;
use crate::file::list_boards;

#[derive(clap::Args, Clone, Debug)]
pub(crate) struct SolveArgs {
    /// Solve boards from file
    source: Option<PathBuf>,
    /// Read board from stdin
    #[arg(short, long, default_value_t = false)]
    stdin: bool,
    /// Maximum iterations of complete board recursion
    #[arg(short, long)]
    max_iterations: Option<usize>,
    /// Limit number of solutions
    #[arg(short, long)]
    limit: Option<usize>,
    /// Select board storage backend
    #[arg(short = 'b', long, value_enum, default_value_t = BoardBackend::Grid)]
    backend: BoardBackend,
}

#[tracing::instrument]
pub fn cmd_solve(args: &SolveArgs) -> Result<()> {
    for board in list_boards(args.source.as_ref(), args.stdin)? {
        dfs_solve_board(board);
    }
    Ok(())
}

fn dfs_solve_board(board: Board) {
    for (index, solution) in board.backtracking(false).enumerate() {
        println!("{}\n{}", index, solution.to_pretty_grid());
    }
}
