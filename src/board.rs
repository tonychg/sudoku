mod bitfield;
mod generator;
mod grid;

pub use bitfield::BitFieldBoard;
pub use grid::GridBoard;

use crate::heuristics::can_be_placed;
use crate::rng;
use itertools::Itertools;
use rand::Rng;
use rand::seq::SliceRandom;
use std::fmt::Debug;

pub trait Board: Send + Sync + Debug {
    fn size(&self) -> usize;
    fn set(&mut self, x: usize, y: usize, num: u8);
    fn get(&self, x: usize, y: usize) -> u8;
    fn next_empty(&self) -> Option<(usize, usize)>;
    fn next_empty_random(&self, y_range: &[usize], x_range: &[usize]) -> Option<(usize, usize)>;
    fn can_be_placed(&self, x: usize, y: usize, num: u8) -> bool;
}

pub fn parse_grid_string(sgrid: impl ToString) -> anyhow::Result<(usize, String)> {
    let sgrid = sgrid.to_string();
    let data = sgrid.split(":").collect::<Vec<&str>>();
    if data.len() != 2 {
        return Err(anyhow::anyhow!("Invalid format size:grid"));
    }
    let size: usize = data[0].parse()?;
    let grid = data[1].to_string();
    Ok((size, grid))
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    board: T,
    visited: bool,
}

impl<T> Node<T>
where
    T: Clone + Board,
{
    pub fn new(board: T) -> Self {
        Self {
            board,
            visited: false,
        }
    }

    pub fn adjacent_nodes(&mut self, y_range: &[usize], x_range: &[usize]) -> Vec<Node<T>> {
        let mut neighbors = Vec::new();

        let position = if y_range.is_empty() && y_range.is_empty() {
            self.board.next_empty()
        } else {
            self.board.next_empty_random(y_range, x_range)
        };

        if let Some((x, y)) = position {
            for num in 1..=9u8 {
                if self.board.can_be_placed(x, y, num) {
                    let mut next_board = self.board.clone();
                    next_board.set(x, y, num);
                    neighbors.push(Self::new(next_board));
                }
            }
        }
        self.visited = true;
        neighbors
    }
}

pub fn solve_dfs<T: Board + Clone>(board: T, seed: Option<u64>, limit: Option<usize>) -> Vec<T> {
    let mut count: usize = 0;
    let mut stack: Vec<Node<T>> = Vec::new();
    let mut solutions: Vec<T> = Vec::new();
    let root = Node::new(board.clone());
    let mut x_range = Vec::new();
    let mut y_range = Vec::new();

    if let Some(seed) = seed {
        let mut rng = rng::rng_from_seed(seed);
        x_range = (0..board.size()).collect::<Vec<usize>>();
        y_range = x_range.clone();
        x_range.shuffle(&mut rng);
        y_range.shuffle(&mut rng);
    }

    stack.push(root);

    while let Some(mut node) = stack.pop() {
        if let Some(limit) = limit {
            if count >= limit {
                return solutions;
            }
        }
        if node.board.next_empty().is_none() {
            count += 1;
            solutions.push(node.board.clone());
        }
        if !node.visited {
            for node in node.adjacent_nodes(&y_range, &x_range) {
                if !node.visited {
                    stack.push(node.clone())
                }
            }
        }
    }

    solutions
}

pub fn to_pretty_grid(board: impl Board) -> String {
    let size = board.size();
    let line = (0..size * 2 + 7)
        .map(|i| if i % 8 == 0 { "+" } else { "-" })
        .fold(String::new(), |acc, c| acc + c);
    let mut output = String::new();
    for y in 0..size {
        for x in 0..size {
            let num = board.get(x, y);
            let elem = if y == 0 && x == 0 {
                format!("{}\n| {}", line, num)
            } else if !(x != size - 1 || y <= 1 && y != size - 1 || y % 3 != 2 && y != size - 1) {
                format!(" {} |\n{}\n", num, line)
            } else if x > 1 && (x - 1) % 3 == 2 {
                format!(" | {}", num)
            } else if x == size - 1 {
                format!(" {} |\n", num)
            } else if x == 0 {
                format!("| {}", num)
            } else {
                format!(" {}", num)
            };
            output.push_str(&elem);
        }
    }
    output
}

pub fn print_pretty(grid: &[Vec<u8>]) {
    let size = grid.len();
    let line = (0..size * 2 + 7)
        .map(|i| if i % 8 == 0 { "+" } else { "-" })
        .fold(String::new(), |acc, c| acc + c);
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if y == 0 && x == 0 {
                print!("{}\n| {}", line, col)
            } else if !(x != size - 1 || y <= 1 && y != size - 1 || y % 3 != 2 && y != size - 1) {
                print!(" {} |\n{}\n", col, line)
            } else if x > 1 && (x - 1) % 3 == 2 {
                print!(" | {}", col)
            } else if x == size - 1 {
                println!(" {} |", col)
            } else if x == 0 {
                print!("| {}", col)
            } else {
                print!(" {}", col)
            }
        }
    }
    println!()
}

pub fn print_raw(grid: &[Vec<u8>]) {
    for row in grid.iter() {
        for col in row.iter() {
            print!("{}", col);
        }
        println!()
    }
}

pub fn solve(grid: &mut Vec<Vec<u8>>, size: usize) -> bool {
    for (x, y) in (0..size).into_iter().tuple_combinations() {
        if grid[y][x] != 0 {
            continue;
        }
        for num in 1..=9u8 {
            if can_be_placed(grid, size, x, y, num) {
                grid[y][x] = num;
                if solve(grid, size) {
                    return true;
                }
            }
        }
        grid[y][x] = 0;
        return false;
    }
    true
}

pub fn make_playable(grid: &[Vec<u8>], starting_numbers: usize, seed: u64) -> Vec<Vec<u8>> {
    let mut holes = Vec::new();
    let mut rng = rng::rng_from_seed(seed);
    let mut playable = grid.to_vec();
    let size = grid.len();
    let total = grid.len() * grid.len();

    while holes.len() < total - starting_numbers {
        let index = rng.random_range(0..total);
        let (x, y) = (index % size, index / size);
        if playable[y][x] == 0 {
            continue;
        }
        holes.push((x, y, grid[y][x]));
        playable[y][x] = 0;
        let node = playable.clone();
        if !solve(&mut node.to_vec(), node.len()) {
            if let Some((x, y, num)) = holes.pop() {
                playable[y][x] = num;
            }
        }
    }
    playable.to_vec()
}

pub fn parse(string: &str) -> Vec<Vec<u8>> {
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in string.split('\n') {
        let mut row: Vec<u8> = Vec::new();
        for char in line.chars() {
            row.push(char.to_string().parse().unwrap())
        }
        grid.push(row);
    }
    grid
}
