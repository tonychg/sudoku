use crate::grid;
use anyhow::Result;
use std::io;

pub fn solve_command(_stdin: bool) -> Result<()> {
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
