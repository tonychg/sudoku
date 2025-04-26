use crate::board::Board;
use crate::board::BoardBackend;
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
            for (index, solution) in dfs::dfs(
                vec![Board::from_str(line, &args.backend)?],
                |b| b.id(),
                |b| b.completed(),
                |b| b.neighbors(),
            )
            .enumerate()
            {
                println!("{}\n{}", index, solution.to_pretty_grid());
            }
        }
    }
    Ok(())
}
