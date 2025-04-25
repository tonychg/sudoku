use super::BoardBackend;
use crate::board::BitFieldBoard;
use crate::board::GridBoard;
use crate::board::to_pretty_grid;
use crate::dfs;
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
    /// Select board storage backend
    #[arg(short = 'b', long, value_enum, default_value_t = BoardBackend::Grid)]
    backend: BoardBackend,
}

#[tracing::instrument]
pub fn cmd_solve(args: &SolveArgs) -> Result<()> {
    if args.stdin {
        for line in io::stdin().lines().map_while(Result::ok) {
            match args.backend {
                BoardBackend::Grid => {
                    for (index, solution) in dfs::dfs(
                        vec![GridBoard::from_str(line)?],
                        |b| b.id(),
                        |b| b.completed(),
                        |b| b.neighbors(),
                    )
                    .enumerate()
                    {
                        println!("{}\n{}", index, to_pretty_grid(&solution));
                    }
                }
                BoardBackend::BitField => {
                    for (index, solution) in dfs::dfs(
                        vec![BitFieldBoard::from_str(line)?],
                        |b| b.id(),
                        |b| b.completed(),
                        |b| b.neighbors(),
                    )
                    .enumerate()
                    {
                        println!("{}\n{}", index, to_pretty_grid(&solution));
                    }
                }
            };
        }
    }
    Ok(())
}
