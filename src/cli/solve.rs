use crate::grid::print_pretty_grid;
use crate::grid::solve;
use anyhow::Result;
use std::io;

pub fn solve_command(stdin: bool) -> Result<()> {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in io::stdin().lines().map_while(Result::ok) {
        let mut row: Vec<u8> = Vec::new();
        for char in line.chars() {
            row.push(char.to_string().parse().unwrap())
        }
        grid.push(row);
    }
    print_pretty_grid(&grid);
    println!();
    solve(&mut grid);
    print_pretty_grid(&grid);
    Ok(())
}
