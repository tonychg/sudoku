use crate::rng;
use rand::Rng;
use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;
use ref_cast::RefCastCustom;
use std::fmt::Display;
use std::ops::Range;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, RefCastCustom, Copy)]
#[repr(transparent)]
pub struct Tile(pub u8);

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = if self.0 == 0 {
            " "
        } else {
            &format!("{}", self.0)
        };
        write!(f, "{}", symbol)
    }
}

impl Tile {
    pub fn init(size: usize) -> Vec<Tile> {
        vec![Tile(0); size * size]
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    pub size: usize,
    pub seed: u64,
    tiles: Vec<Tile>,
}

impl Board {
    pub fn new(size: usize, seed: u64) -> Self {
        Self {
            size,
            seed,
            tiles: Tile::init(size),
        }
    }

    pub fn clear(&mut self) {
        self.tiles = Tile::init(self.size)
    }
}

pub fn format_grid(i: usize, tile: &Tile, size: usize) -> String {
    let x = i % size;
    let y = i / size;
    let line = (0..size * 2 + 7)
        .map(|i| if i % 8 == 0 { "+" } else { "-" })
        .fold(String::new(), |acc, c| acc + c);

    if y == 0 && x == 0 {
        format!("{}\n| {}", line, tile)
    } else if !(x != size - 1 || y <= 1 && y != size - 1 || y % 3 != 2 && y != size - 1) {
        format!(" {} |\n{}\n", tile, line)
    } else if x > 1 && (x - 1) % 3 == 2 {
        format!(" | {}", tile)
    } else if x == size - 1 {
        format!(" {} |\n", tile)
    } else if x == 0 {
        format!("| {}", tile)
    } else {
        format!(" {}", tile)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid = self
            .tiles
            .iter()
            .enumerate()
            .map(|(i, tile)| format_grid(i, tile, self.size))
            .fold(String::new(), |acc, c| acc + &c);
        write!(f, "{}", grid)
    }
}

pub fn index_from_xy(x: usize, y: usize, size: usize) -> usize {
    x + y * size
}

pub fn index_to_xy(index: usize, size: usize) -> (usize, usize) {
    (index % size, index / size)
}

pub fn is_in_column(x: usize, board: &Board, value: u8) -> bool {
    (0..board.size)
        .map(|i| board.tiles[index_from_xy(x, i, board.size)])
        .filter(|v| *v == Tile(value))
        .count()
        != 0
}

pub fn is_in_row(y: usize, board: &Board, value: u8) -> bool {
    (0..board.size)
        .map(|i| board.tiles[index_from_xy(i, y, board.size)])
        .filter(|v| *v == Tile(value))
        .count()
        != 0
}

pub fn is_in_square(x: usize, y: usize, board: &Board, value: u8) -> bool {
    let third = board.size / 3;
    (0..board.size)
        .map(|i| {
            board.tiles[index_from_xy(
                (x / third * third) + i % third,
                (y / third * third) + i / third,
                board.size,
            )]
        })
        .filter(|v| *v == Tile(value))
        .count()
        != 0
}

pub fn is_placeable(x: usize, y: usize, board: &Board, value: u8) -> bool {
    !is_in_column(x, board, value)
        && !is_in_row(y, board, value)
        && !is_in_square(x, y, board, value)
}

pub fn search_empty_tiles(board: &Board) -> Vec<usize> {
    board
        .tiles
        .iter()
        .enumerate()
        .filter(|(_, v)| **v == Tile(0))
        .map(|(i, _)| i)
        .collect()
}

pub fn shuffled_empty_tiles(board: &Board, rng: &mut ChaCha8Rng) -> Vec<usize> {
    let mut empty_tiles = search_empty_tiles(board);
    empty_tiles.shuffle(rng);
    empty_tiles
}

pub fn next_empty(board: &Board, rng: &mut ChaCha8Rng) -> Option<(usize, usize)> {
    let mut x_range = (0..board.size).collect::<Vec<usize>>();
    let mut y_range = x_range.clone();

    x_range.shuffle(rng);
    y_range.shuffle(rng);

    for y in &y_range {
        for x in &x_range {
            if board.tiles[index_from_xy(*x, *y, board.size)] == Tile(0) {
                return Some((*x, *y));
            }
        }
    }
    None
}

pub fn solve(board: &Board, rng: &mut ChaCha8Rng, counter: &mut usize) -> Option<Board> {
    let mut numbers: Vec<u8> = (1..=9).collect();
    let (x, y) = match next_empty(board, rng) {
        None => return Some(board.clone()),
        Some((x, y)) => (x, y),
    };

    numbers.shuffle(rng);

    for number in &numbers {
        *counter += 1;
        if *counter >= 5000 {
            return None;
        }
        if is_placeable(x, y, board, *number) {
            let mut board = board.clone();
            board.tiles[index_from_xy(x, y, board.size)] = Tile(*number);
            if let Some(board) = solve(&board, rng, counter) {
                return Some(board.clone());
            }
            board.tiles[index_from_xy(x, y, board.size)] = Tile(0);
        }
    }
    None
}

pub struct BoardGenerator {
    max_iterations: usize,
    rng: ChaCha8Rng,
    pub board: Board,
    iterations: usize,
    is_cut: bool,
}

impl BoardGenerator {
    pub fn builder() -> BoardGeneratorBuilder {
        BoardGeneratorBuilder::default()
    }

    pub fn reset(&mut self) {
        self.iterations = 0;
        self.board.seed = self.rng.random();
        self.board.clear();
    }

    fn next(&mut self) -> Board {
        self.rng = rng::rng_from_seed(self.board.seed);

        self.iterations += 1;

        if self.iterations >= self.max_iterations {
            self.reset();
        }

        let (x, y) = match next_empty(&self.board, &mut self.rng) {
            None => {
                self.is_cut = false;
                return self.board.clone();
            }
            Some((x, y)) => (x, y),
        };

        for value in 1..=self.board.size as u8 {
            if is_placeable(x, y, &self.board, value) {
                self.board.tiles[index_from_xy(x, y, self.board.size)] = Tile(value);
                let board = self.next();
                if !self.is_cut {
                    return board;
                }
                self.board.tiles[index_from_xy(x, y, board.size)] = Tile(0);
            }
        }
        self.is_cut = true;
        self.board.clone()
    }

    pub fn board(&mut self) -> Board {
        self.next()
    }

    pub fn make_playable(&mut self, number_of_holes: usize) -> PlayableBoard {
        let mut holes = Vec::new();
        let mut order = 0;

        while holes.len() < number_of_holes {
            let index = self.rng.random_range(0..self.board.size * self.board.size);
            let (x, y) = index_to_xy(index, self.board.size);
            if self.board.tiles[index] == Tile(0) {
                continue;
            }
            holes.push(Hole {
                x,
                y,
                order,
                value: self.board.tiles[index],
            });
            self.board.tiles[index] = Tile(0);
            let board = self.board.clone();
            if solve(&board, &mut self.rng, &mut 0).is_none() {
                if let Some(hole) = holes.pop() {
                    self.board.tiles[index] = hole.value;
                }
            } else {
                println!("order: {} holes: {}", order, holes.len());
                println!("{}", self.board);
                order += 1;
                // println!("Board is still solvable");
            }
        }
        PlayableBoard {
            board: self.board.clone(),
            holes: holes.clone(),
        }
    }
}

pub struct BoardGeneratorBuilder {
    seed: u64,
    max_iterations: usize,
    size: usize,
}

impl Default for BoardGeneratorBuilder {
    fn default() -> Self {
        Self {
            seed: rng::generate_seed(),
            size: 9,
            max_iterations: 500,
        }
    }
}

impl BoardGeneratorBuilder {
    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    pub fn max_iterations(mut self, max_iterations: usize) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    pub fn size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }

    pub fn build(self) -> BoardGenerator {
        BoardGenerator {
            max_iterations: self.max_iterations,
            board: Board::new(self.size, self.seed),
            rng: rng::rng_from_seed(self.seed),
            iterations: 0,
            is_cut: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Hole {
    x: usize,
    y: usize,
    order: usize,
    value: Tile,
}

#[derive(Debug, Clone)]
pub struct PlayableBoard {
    pub board: Board,
    pub holes: Vec<Hole>,
}

#[derive(Debug, Clone)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn starting_range(&self) -> Range<usize> {
        match self {
            Difficulty::Easy => 35..40,
            Difficulty::Medium => 25..35,
            Difficulty::Hard => 21..25,
        }
    }
}
