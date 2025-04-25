use super::Board;
use super::parse_grid_string;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct GridBoard {
    size: usize,
    grid: Vec<u8>,
}

impl GridBoard {
    pub fn from_str(sboard: impl ToString) -> anyhow::Result<Self> {
        let (size, grid) = parse_grid_string(sboard)?;
        let mut board = Self::new(size);
        for (index, num) in grid.chars().enumerate() {
            let (x, y) = board.xy(index);
            let num: u8 = num.to_string().parse()?;
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

    fn in_column(&self, x: usize, num: u8) -> bool {
        for y in 0..self.size {
            if self.grid[self.index(x, y)] == num {
                return true;
            }
        }
        false
    }

    fn in_row(&self, y: usize, num: u8) -> bool {
        for x in 0..self.size {
            if self.grid[self.index(x, y)] == num {
                return true;
            }
        }
        false
    }

    fn in_quad(&self, x: usize, y: usize, num: u8) -> bool {
        let third = self.size / 3;
        let first_x = x / third * third;
        let first_y = y / third * third;
        for index in (0..self.size).map(|i| self.index(first_x + i % third, first_y + i / third)) {
            if self.grid[index] == num {
                return true;
            }
        }
        false
    }
}

impl Board for GridBoard {
    fn new(size: usize) -> Self {
        Self {
            size,
            grid: vec![0; size * size],
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn next_empty(&self) -> Option<(usize, usize)> {
        for (index, num) in self.grid.iter().enumerate() {
            if *num == 0 {
                return Some(self.xy(index));
            }
        }
        None
    }

    fn next_empty_random(&self, y_range: &[usize], x_range: &[usize]) -> Option<(usize, usize)> {
        for y in y_range.iter() {
            for x in x_range.iter() {
                let index = self.index(*x, *y);
                if self.grid[index] == 0 {
                    return Some((*x, *y));
                }
            }
        }
        None
    }

    fn set(&mut self, x: usize, y: usize, num: u8) {
        let index = self.index(x, y);
        self.grid[index] = num;
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.grid[self.index(x, y)]
    }

    fn can_be_placed(&self, x: usize, y: usize, num: u8) -> bool {
        !self.in_column(x, num) && !self.in_row(y, num) && !self.in_quad(x, y, num)
    }
}

impl Display for GridBoard {
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
