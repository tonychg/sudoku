use crate::rng;
use rand::Rng;
use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;

fn in_column(grid: &[Vec<u8>], x: usize, value: u8) -> bool {
    (0..grid.len())
        .map(|i| grid[i][x])
        .filter(|v| *v == value)
        .count()
        != 0
}

fn in_row(grid: &[Vec<u8>], y: usize, value: u8) -> bool {
    (0..grid.len())
        .map(|i| grid[y][i])
        .filter(|v| *v == value)
        .count()
        != 0
}

fn in_square(grid: &[Vec<u8>], x: usize, y: usize, value: u8) -> bool {
    let third = grid.len() / 3;
    (0..grid.len())
        .map(|i| grid[(y / third * third) + i / third][(x / third * third) + i % third])
        .filter(|v| *v == value)
        .count()
        != 0
}

fn is_placeable(grid: &[Vec<u8>], x: usize, y: usize, value: u8) -> bool {
    !in_column(grid, x, value) && !in_row(grid, y, value) && !in_square(grid, x, y, value)
}

fn next_empty(grid: &[Vec<u8>], rng: &mut ChaCha8Rng) -> Option<(usize, usize)> {
    let size = grid.len();
    let mut x_range = (0..size).collect::<Vec<usize>>();
    let mut y_range = (0..size).collect::<Vec<usize>>();

    x_range.shuffle(rng);
    y_range.shuffle(rng);

    for y in y_range.iter() {
        for x in x_range.iter() {
            if grid[*y][*x] == 0 {
                return Some((*x, *y));
            }
        }
    }

    None
}

pub fn init(grid: &mut Vec<Vec<u8>>, size: usize) {
    grid.clear();

    for _ in 0..size {
        grid.push(vec![0; size]);
    }
}

pub fn place_number(grid: &mut [Vec<u8>], x: usize, y: usize) -> Option<(usize, usize)> {
    for i in 1..=9u8 {
        if is_placeable(grid, x, y, i) {
            grid[y][x] = i;
            return Some((x, y));
        }
    }
    grid[y][x] = 0;
    None
}

pub fn count_empty(grid: &[Vec<u8>]) -> usize {
    let mut counter = 0;
    for row in grid.iter() {
        for value in row.iter() {
            if *value == 0 {
                counter += 1;
            }
        }
    }
    counter
}

pub fn print_pretty_grid(grid: &[Vec<u8>]) {
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

pub fn print_raw_grid(grid: &[Vec<u8>]) {
    for row in grid.iter() {
        for col in row.iter() {
            print!("{}", col);
        }
        println!()
    }
}

pub fn try_to_place_num(grid: &mut [Vec<u8>], x: usize, y: usize) -> bool {
    for num in 1..=9u8 {
        if is_placeable(grid, x, y, num) {
            grid[y][x] = num;
            return true;
        }
    }
    false
}

pub struct Context {
    max_iterations: usize,
    rng: ChaCha8Rng,
    grid: Vec<Vec<u8>>,
    size: usize,
    iterations: usize,
    is_cut: bool,
    seed: u64,
}

pub fn generate_complete_grid(ctx: &mut Context) -> Vec<Vec<u8>> {
    ctx.rng = rng::rng_from_seed(ctx.seed);
    ctx.iterations += 1;
    if ctx.iterations >= ctx.max_iterations {
        ctx.iterations = 0;
        ctx.seed = ctx.rng.random();
        ctx.grid = vec![vec![0; ctx.size]; ctx.size];
    }

    let (x, y) = match next_empty(&ctx.grid, &mut ctx.rng) {
        Some((x, y)) => (x, y),
        None => {
            ctx.is_cut = false;
            return ctx.grid.clone();
        }
    };

    for num in 1..=9u8 {
        if is_placeable(&ctx.grid, x, y, num) {
            ctx.grid[y][x] = num;
            let grid = generate_complete_grid(ctx);
            if !ctx.is_cut {
                return grid;
            }
            ctx.grid[y][x] = 0;
        }
    }
    ctx.is_cut = true;
    ctx.grid.clone()
}

pub fn generate(size: usize, seed: u64, max_iterations: usize) -> Vec<Vec<u8>> {
    let mut ctx = Context {
        rng: rng::rng_from_seed(seed),
        size,
        grid: vec![vec![0; size]; size],
        is_cut: false,
        max_iterations,
        seed,
        iterations: 0,
    };

    generate_complete_grid(&mut ctx)
}

pub fn solve(grid: &mut Vec<Vec<u8>>) -> bool {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != 0 {
                continue;
            }
            for i in 1..=9u8 {
                if is_placeable(grid, x, y, i) {
                    grid[y][x] = i;
                    if solve(grid) {
                        return true;
                    }
                }
            }
            grid[y][x] = 0;
            return false;
        }
    }
    true
}

pub fn make_playable(grid: &[Vec<u8>], starting_numbers: usize, seed: u64) -> Vec<Vec<u8>> {
    let mut holes = Vec::new();
    let size = grid.len();
    let total = grid.len() * grid.len();
    let mut rng = rng::rng_from_seed(seed);
    let mut playable = grid.to_vec();

    while holes.len() < total - starting_numbers {
        let index = rng.random_range(0..total);
        let (x, y) = (index % size, index / size);
        if playable[y][x] == 0 {
            continue;
        }
        holes.push((x, y, grid[y][x]));
        playable[y][x] = 0;
        let node = playable.clone();
        if !solve(&mut node.to_vec()) {
            if let Some((x, y, num)) = holes.pop() {
                playable[y][x] = num;
            }
        }
    }
    playable.to_vec()
}
