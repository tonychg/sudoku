use crate::grid;
use anyhow::Result;
use std::io;

#[derive(clap::Args, Clone, Debug)]
pub(crate) struct SolveArgs {
    #[arg(short, long, default_value_t = true)]
    stdin: bool,
}

#[tracing::instrument]
pub fn cmd_solve(args: &SolveArgs) -> Result<()> {
    let mut buffer = String::new();
    if args.stdin {
        buffer = io::stdin()
            .lines()
            .map_while(Result::ok)
            .collect::<Vec<String>>()
            .join("\n");
    }
    let mut grid = grid::parse(&buffer);
    let size = grid.len();
    grid::print_pretty(&grid);
    println!();
    grid::solve(&mut grid, size);
    grid::print_pretty(&grid);
    Ok(())
}
