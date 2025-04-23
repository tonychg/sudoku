use crate::board;
use anyhow::Result;
use std::io;

#[derive(clap::ValueEnum, Default, Debug, Clone)]
pub enum Solver {
    #[default]
    Recursive,
    Dfs,
}

#[derive(clap::Args, Clone, Debug)]
pub(crate) struct SolveArgs {
    #[arg(short, long, default_value_t = true)]
    stdin: bool,
    #[arg(short = 'S', long, value_enum, default_value_t = Solver::Recursive)]
    solver: Solver,
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
    let mut grid = board::parse(&buffer);
    match args.solver {
        Solver::Recursive => solve_recursive(&mut grid),
        Solver::Dfs => solve_dfs(&mut grid),
    }
    Ok(())
}

fn solve_recursive(grid: &[Vec<u8>]) {
    let size = grid.len();
    let mut grid = grid.to_vec();
    board::print_pretty(&grid);
    println!();
    board::solve(&mut grid, size);
    board::print_pretty(&grid);
}

fn solve_dfs(grid: &[Vec<u8>]) {
    let size = grid.len();
    let node = board::dfs::Node::new(grid, size);
    board::print_pretty(&grid);
    println!();
    let solutions = board::dfs::dfs(node, None);
    if solutions.is_empty() {
        println!("No solution found");
    } else {
        for solution in solutions.iter() {
            board::print_pretty(&solution.grid);
        }
        println!("Found {} solutions", solutions.len());
    }
}
