use crate::rng;
use itertools::Itertools;
use rand::Rng;
use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;

fn in_column(grid: &[Vec<u8>], size: usize, x: usize, value: u8) -> bool {
    for y in 0..size {
        if grid[y][x] == value {
            return true;
        }
    }
    false
}

fn in_row(grid: &[Vec<u8>], size: usize, y: usize, value: u8) -> bool {
    for x in 0..size {
        if grid[y][x] == value {
            return true;
        }
    }
    false
}

fn in_square(grid: &[Vec<u8>], size: usize, x: usize, y: usize, value: u8) -> bool {
    let third = grid.len() / 3;
    let first_x = x / third * third;
    let first_y = y / third * third;

    for (x, y) in (0..size).map(|i| (first_x + i % third, first_y + i / third)) {
        if grid[y][x] == value {
            return true;
        }
    }
    false
}

fn is_placeable(grid: &[Vec<u8>], size: usize, x: usize, y: usize, value: u8) -> bool {
    !in_column(grid, size, x, value)
        && !in_row(grid, size, y, value)
        && !in_square(grid, size, x, y, value)
}

fn next_empty(grid: &[Vec<u8>], y_range: &[usize], x_range: &[usize]) -> Option<(usize, usize)> {
    for y in y_range.iter() {
        for x in x_range.iter() {
            if grid[*y][*x] == 0 {
                return Some((*x, *y));
            }
        }
    }

    None
}

struct GridGenerator {
    max_iterations: usize,
    rng: ChaCha8Rng,
    grid: Vec<Vec<u8>>,
    size: usize,
    iterations: usize,
    is_cut: bool,
    seed: u64,
    y_range: Vec<usize>,
    x_range: Vec<usize>,
}

impl GridGenerator {
    fn init(&mut self) {
        self.x_range = (0..self.size).collect::<Vec<usize>>();
        self.y_range = (0..self.size).collect::<Vec<usize>>();

        self.x_range.shuffle(&mut self.rng);
        self.y_range.shuffle(&mut self.rng);
    }

    pub fn grid(&mut self) -> Vec<Vec<u8>> {
        if self.iterations == 0 {
            self.init()
        }

        if self.iterations >= self.max_iterations {
            self.iterations = 1;
            self.seed = self.rng.random();
            self.grid = vec![vec![0; self.size]; self.size];
        }

        self.rng = rng::rng_from_seed(self.seed);
        self.iterations += 1;

        self.x_range.shuffle(&mut self.rng);
        self.y_range.shuffle(&mut self.rng);

        let (x, y) = match next_empty(&self.grid, &self.y_range, &self.x_range) {
            Some((x, y)) => (x, y),
            None => {
                self.is_cut = false;
                return self.grid.clone();
            }
        };
        for num in 1..=9u8 {
            if is_placeable(&self.grid, self.size, x, y, num) {
                self.grid[y][x] = num;
                let grid = self.grid();
                if !self.is_cut {
                    return grid;
                }
                self.grid[y][x] = 0;
            }
        }
        self.is_cut = true;
        self.grid.clone()
    }
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

pub fn generate(size: usize, seed: u64, max_iterations: usize) -> Vec<Vec<u8>> {
    let mut ctx = GridGenerator {
        rng: rng::rng_from_seed(seed),
        size,
        grid: vec![vec![0; size]; size],
        is_cut: false,
        max_iterations,
        seed,
        iterations: 0,
        x_range: Vec::new(),
        y_range: Vec::new(),
    };

    ctx.grid()
}

pub fn solve(grid: &mut Vec<Vec<u8>>, size: usize) -> bool {
    for (x, y) in (0..size).into_iter().tuple_combinations() {
        if grid[y][x] != 0 {
            continue;
        }
        for num in 1..=9u8 {
            if is_placeable(grid, size, x, y, num) {
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
