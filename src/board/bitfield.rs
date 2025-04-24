use super::Board;
use super::parse_grid_string;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct BitFieldBoard {
    size: usize,
    grid: Vec<u8>,
    rows: u128,
    cols: u128,
    quads: u128,
}

impl BitFieldBoard {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            rows: 0,
            cols: 0,
            quads: 0,
            grid: vec![0; size * size],
        }
    }

    fn from_str(sboard: impl ToString) -> anyhow::Result<Self> {
        let (size, grid) = parse_grid_string(sboard)?;
        let mut board = Self::new(size);
        for (index, num) in grid.chars().enumerate() {
            let (x, y) = board.xy(index);
            let num = num as u8;
            board.set(x, y, num);
        }
        Ok(board)
    }

    fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.size
    }

    fn xy(&self, index: usize) -> (usize, usize) {
        (index % self.size, index / self.size)
    }
}

impl Board for BitFieldBoard {
    fn size(&self) -> usize {
        self.size
    }

    fn next_empty_random(&self, y_range: &[usize], x_range: &[usize]) -> Option<(usize, usize)> {
        todo!()
    }

    fn next_empty(&self) -> Option<(usize, usize)> {
        for (index, num) in self.grid.iter().enumerate() {
            if *num == 0 {
                return Some(self.xy(index));
            }
        }
        None
    }

    fn set(&mut self, x: usize, y: usize, num: u8) {
        let third = self.size / 3;
        let index = self.index(x, y);
        self.rows |= 1 << (x * self.size + num as usize);
        self.cols |= 1 << (y * self.size + num as usize);
        self.quads |= 1 << ((y / third * third + x / third) * self.size + num as usize);
        self.grid[index] = num;
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.grid[self.index(x, y)]
    }

    fn can_be_placed(&self, x: usize, y: usize, num: u8) -> bool {
        let b = (y / 3) * 3 + (x / 3);
        let mask = (self.rows >> (y * 9)) | (self.cols >> (x * 9)) | (self.quads >> (b * 9));
        let mask_num = 1 << num;
        (mask & mask_num) == 0
    }
}

impl Display for BitFieldBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid_line = self
            .grid
            .iter()
            .map(|num| format!("{}", num))
            .collect::<Vec<String>>()
            .join("");
        write!(f, "{}:{}", self.size, grid_line)
    }
}
