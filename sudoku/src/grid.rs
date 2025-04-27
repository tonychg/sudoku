use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use rand::Rng;
use rand::seq::SliceRandom;

use crate::rng;

pub const SIZE: usize = 9;
const ONE_THIRD: usize = 3;

#[derive(Clone, Debug)]
pub struct Grid {
    pub seed: u64,
    pub digits: Vec<u8>,
    rows: u128,
    cols: u128,
    quads: u128,
}

impl Grid {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            digits: vec![0; SIZE * SIZE],
            rows: 0,
            cols: 0,
            quads: 0,
        }
    }

    pub fn from_other(other: &Grid) -> Self {
        let mut rng = rng::rng_from_seed(other.seed);
        Self {
            seed: rng.random(),
            digits: other.digits.clone(),
            rows: other.rows,
            cols: other.cols,
            quads: other.quads,
        }
    }

    pub fn id(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.digits.hash(&mut s);
        s.finish()
    }

    pub fn next_empty(&self) -> Option<(usize, usize)> {
        for (index, num) in self.digits.iter().enumerate() {
            if *num == 0 {
                return Some((index % SIZE, index / SIZE));
            }
        }
        None
    }

    pub fn from_str(input: impl ToString) -> anyhow::Result<Self> {
        let (_, seed, input) = parse_grid_string(input)?;
        let mut grid = Self::new(seed);
        for (index, num) in input.chars().enumerate() {
            let (x, y) = (index % SIZE, index / SIZE);
            let num: u8 = num.to_string().parse()?;
            grid.set_digit(x, y, num);
        }
        Ok(grid)
    }

    pub fn set_digit(&mut self, x: usize, y: usize, num: u8) {
        let size = SIZE;
        let third = ONE_THIRD;
        let quads_key = (y / third * third + x / third) * size;
        let cols_key = x * size;
        let rows_key = y * size;
        if self.digits[y * SIZE + x] > 0 {
            let actual_key = 1 << (self.digits[y * SIZE + x] - 1);
            self.cols ^= actual_key << cols_key;
            self.rows ^= actual_key << rows_key;
            self.quads ^= actual_key << quads_key;
        }
        if num != 0 {
            let key = 1 << (num as usize - 1);
            self.cols |= key << cols_key;
            self.rows |= key << rows_key;
            self.quads |= key << quads_key;
        }
        self.digits[y * SIZE + x] = num;
    }

    pub fn get_digit(&self, x: usize, y: usize) -> u8 {
        self.digits[y * SIZE + x]
    }

    fn can_be_place(&self, x: usize, y: usize, num: u8) -> bool {
        let size = SIZE;
        let third = ONE_THIRD;
        let key = 1 << (num as usize - 1);
        let rows_mask = self.rows >> y * size;
        let cols_mask = self.cols >> x * size;
        let quads_mask = self.quads >> ((y / third * third + x / third) * size);
        let mask = rows_mask | cols_mask | quads_mask;
        (mask & key) == 0
    }

    fn create_neighbors(&self, x: usize, y: usize) -> Vec<Self> {
        let mut neighbors = Vec::new();
        for num in 1..=9u8 {
            if self.can_be_place(x, y, num) {
                let mut next_board = Self::from_other(self);
                next_board.set_digit(x, y, num);
                neighbors.push(next_board);
            }
        }
        neighbors
    }

    pub fn completed(&self) -> bool {
        !self.next_empty().is_some()
    }

    pub fn neighbors(&self) -> Vec<Self> {
        let mut neighbors = Vec::new();
        if let Some((x, y)) = self.next_empty() {
            neighbors = self.create_neighbors(x, y)
        }
        neighbors
    }

    pub fn random_neighbors(&self) -> Vec<Self> {
        let mut rng = rng::rng_from_seed(self.seed);
        let mut neighbors = Vec::new();
        if let Some((x, y)) = self.next_empty() {
            neighbors = self.create_neighbors(x, y)
        }
        neighbors.shuffle(&mut rng);
        neighbors
    }

    pub fn to_pretty_string(&self) -> String {
        let size = SIZE;
        let line = (0..size * 2 + 7)
            .map(|i| if i % 8 == 0 { "+" } else { "-" })
            .fold(String::new(), |acc, c| acc + c);
        let mut output = String::new();
        for y in 0..size {
            for x in 0..size {
                let num = self.get_digit(x, y);
                let num_char = if num == 0 { " " } else { &format!("{}", num) };
                let elem = if y == 0 && x == 0 {
                    format!("{}\n| {}", line, num_char)
                } else if !(x != size - 1 || y <= 1 && y != size - 1 || y % 3 != 2 && y != size - 1)
                {
                    format!(" {} |\n{}\n", num_char, line)
                } else if x > 1 && (x - 1) % 3 == 2 {
                    format!(" | {}", num_char)
                } else if x == size - 1 {
                    format!(" {} |\n", num_char)
                } else if x == 0 {
                    format!("| {}", num_char)
                } else {
                    format!(" {}", num_char)
                };
                output.push_str(&elem);
            }
        }
        output
    }
}

fn parse_grid_string(string_grid: impl ToString) -> anyhow::Result<(usize, u64, String)> {
    let string_grid = string_grid.to_string();
    let data = string_grid.split(":").collect::<Vec<&str>>();
    if data.len() != 3 {
        return Err(anyhow::anyhow!("Invalid format size:seed:grid"));
    }
    let size: usize = data[0].parse()?;
    let seed: u64 = data[1].parse()?;
    let grid = data[2].to_string();
    Ok((size, seed, grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_set_digit() {
        let mut grid = Grid::new(0);
        grid.set_digit(0, 1, 8);
        assert_eq!(grid.digits[9], 8);
        assert_eq!(grid.rows, 0b10000000000000000);
        assert_eq!(grid.cols, 0b10000000);
        assert_eq!(grid.quads, 0b10000000);
    }

    #[test]
    fn test_grid_replace_digit() {
        let mut grid = Grid::new(0);
        grid.set_digit(0, 1, 8);
        grid.set_digit(2, 1, 5);
        grid.set_digit(2, 1, 0);
        assert_eq!(grid.digits[9], 8);
        assert_eq!(grid.rows, 0b10000000000000000);
        assert_eq!(grid.cols, 0b10000000);
        assert_eq!(grid.quads, 0b10000000);
    }

    fn setup_grid(input: Vec<Vec<u8>>) -> Grid {
        let mut grid = Grid::new(0);
        for (j, row) in input.iter().enumerate() {
            for (i, num) in row.iter().enumerate() {
                grid.set_digit(i, j, *num);
            }
        }
        grid
    }

    #[test]
    fn test_grid_heuristic_quad() {
        let grid = setup_grid(vec![
            vec![0, 2, 3, 0, 0, 0, 0, 0, 0],
            vec![1, 5, 4, 0, 0, 0, 0, 0, 0],
            vec![6, 7, 8, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        ]);
        assert_eq!(grid.can_be_place(0, 0, 9), true);
        assert_eq!(grid.can_be_place(0, 0, 1), false);
        assert_eq!(grid.can_be_place(0, 0, 2), false);
        assert_eq!(grid.can_be_place(0, 0, 3), false);
        assert_eq!(grid.can_be_place(0, 0, 4), false);
        assert_eq!(grid.can_be_place(0, 0, 5), false);
        assert_eq!(grid.can_be_place(0, 0, 6), false);
        assert_eq!(grid.can_be_place(0, 0, 7), false);
        assert_eq!(grid.can_be_place(0, 0, 8), false);
    }

    #[test]
    fn test_grid_heuristic_quad_mutiple_replace() {
        let mut grid = setup_grid(vec![
            vec![0, 2, 3, 0, 0, 0, 0, 0, 0],
            vec![1, 5, 4, 0, 0, 0, 0, 0, 0],
            vec![6, 7, 8, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        ]);
        grid.set_digit(1, 3, 6);
        grid.set_digit(0, 3, 5);
        grid.set_digit(0, 0, 9);
        grid.set_digit(0, 0, 0);
        assert_eq!(grid.can_be_place(0, 0, 9), true);
        assert_eq!(grid.can_be_place(0, 0, 1), false);
        assert_eq!(grid.can_be_place(0, 0, 2), false);
        assert_eq!(grid.can_be_place(0, 0, 3), false);
        assert_eq!(grid.can_be_place(0, 0, 4), false);
        assert_eq!(grid.can_be_place(0, 0, 5), false);
        assert_eq!(grid.can_be_place(0, 0, 6), false);
        assert_eq!(grid.can_be_place(0, 0, 7), false);
        assert_eq!(grid.can_be_place(0, 0, 8), false);
    }

    #[test]
    fn test_grid_heuristic_row() {
        let grid = setup_grid(vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![9, 8, 7, 6, 5, 4, 3, 2, 0],
        ]);
        assert_eq!(grid.can_be_place(0, 8, 1), true);
        assert_eq!(grid.can_be_place(0, 8, 2), false);
        assert_eq!(grid.can_be_place(0, 8, 3), false);
        assert_eq!(grid.can_be_place(0, 8, 4), false);
        assert_eq!(grid.can_be_place(0, 8, 5), false);
        assert_eq!(grid.can_be_place(0, 8, 6), false);
        assert_eq!(grid.can_be_place(0, 8, 7), false);
        assert_eq!(grid.can_be_place(0, 8, 8), false);
        assert_eq!(grid.can_be_place(0, 8, 9), false);
    }

    #[test]
    fn test_grid_heuristic_col() {
        let grid = setup_grid(vec![
            vec![0, 0, 0, 0, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 2, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 9, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 8, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 5, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 4, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 3, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 7, 0, 0, 0, 0],
        ]);
        assert_eq!(grid.can_be_place(4, 4, 6), true);
        assert_eq!(grid.can_be_place(4, 4, 1), false);
        assert_eq!(grid.can_be_place(4, 4, 2), false);
        assert_eq!(grid.can_be_place(4, 4, 3), false);
        assert_eq!(grid.can_be_place(4, 4, 4), false);
        assert_eq!(grid.can_be_place(4, 4, 5), false);
        assert_eq!(grid.can_be_place(4, 4, 7), false);
        assert_eq!(grid.can_be_place(4, 4, 8), false);
        assert_eq!(grid.can_be_place(4, 4, 9), false);
    }
}
