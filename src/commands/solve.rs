use crate::grid;
use anyhow::Result;
use std::io;

#[derive(clap::Args, Clone, Debug)]
pub(crate) struct SolveArgs {
    #[arg(short, long, default_value_t = false)]
    stdin: bool,
}

#[tracing::instrument]
pub fn cmd_solve(_args: &SolveArgs) -> Result<()> {
    let buffer = io::stdin()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>()
        .join("\n");
    let mut grid = grid::parse(&buffer);
    grid::print_pretty(&grid);
    println!();
    grid::solve(&mut grid);
    grid::print_pretty(&grid);
    Ok(())
}
