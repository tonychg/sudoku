use std::io;
use std::path::PathBuf;

use anyhow::Result;

use crate::board::Board;
use crate::board::BoardBackend;
use crate::dfs;
use crate::file::read_boards;

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

fn dfs_solve_board(board: Board) {
    for (index, solution) in dfs::dfs(
        vec![board],
        |b| b.id(),
        |b| b.completed(),
        |b| b.neighbors(),
    )
    .enumerate()
    {
        println!("{}\n{}", index, solution.to_pretty_grid());
    }
}

#[tracing::instrument]
pub fn cmd_solve(args: &SolveArgs) -> Result<()> {
    if let Some(source) = &args.source {
        for board in read_boards(&source.as_path())? {
            dfs_solve_board(board);
        }
    }
    if args.stdin {
        for board in io::stdin()
            .lines()
            .map_while(Result::ok)
            .map(|line| Board::from_str(line, &args.backend))
        {
            dfs_solve_board(board?);
        }
    }
    Ok(())
}
