use super::Board;
use super::parse_grid_string;
use std::fmt::Display;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct BitField {
    size: usize,
    field: u128,
}

impl BitField {
    pub fn new(size: usize) -> Self {
        Self { size, field: 0 }
    }

    pub fn set(&mut self, index: usize, num: u8) {
        self.field |= (1 << (num + 1) as u128) << index * self.size;
    }

    pub fn get(&mut self, index: usize) -> u8 {
        (self.field >> (index * self.size) & 0xff) as u8
    }
}

impl Display for BitField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#081b}", self.field)
    }
}

#[derive(Debug)]
pub struct BitFieldBoard {
    id: Uuid,
    size: usize,
    seed: u64,
    grid: Vec<u8>,
    rows: u128,
    cols: u128,
    quads: u128,
}

impl BitFieldBoard {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn from_str(sboard: impl ToString) -> anyhow::Result<Self> {
        let (size, seed, grid) = parse_grid_string(sboard)?;
        let mut board = Self::new(size, seed);
        for (index, num) in grid.chars().enumerate() {
            let (x, y) = board.xy(index);
            let num = num.to_string().parse()?;
            if num != 0 {
                board.set(x, y, num);
            }
        }
        Ok(board)
    }

    pub fn from_board(board: &BitFieldBoard) -> Self {
        Self {
            id: Uuid::new_v4(),
            size: board.size,
            seed: board.seed,
            grid: board.grid.clone(),
            rows: board.rows,
            cols: board.cols,
            quads: board.quads,
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.size
    }

    fn xy(&self, index: usize) -> (usize, usize) {
        (index % self.size, index / self.size)
    }

    pub fn completed(&self) -> bool {
        self.next_empty().is_none()
    }

    pub fn neighbors(&self) -> Vec<BitFieldBoard> {
        let mut neighbors = Vec::new();
        if let Some((x, y)) = self.next_empty() {
            for num in 1..=9u8 {
                if self.can_be_placed(x, y, num) {
                    let mut next_board = Self::from_board(self);
                    next_board.set(x, y, num);
                    neighbors.push(next_board);
                }
            }
        }
        neighbors
    }
}

impl Board for BitFieldBoard {
    fn new(size: usize, seed: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            size,
            seed,
            rows: 0,
            cols: 0,
            quads: 0,
            grid: vec![0; size * size],
        }
    }

    fn seed(&self) -> u64 {
        self.seed
    }

    fn size(&self) -> usize {
        self.size
    }

    fn next_empty_random(&self, y_range: &[usize], x_range: &[usize]) -> Option<(usize, usize)> {
        for y in y_range {
            for x in x_range {
                if self.get(*x, *y) == 0 {
                    return Some((*x, *y));
                }
            }
        }
        None
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
        self.grid[index] = num;
        let num = 1 << (num + 1) as u128;
        self.rows |= num << (y * self.size);
        self.cols |= num << (x * self.size);
        self.quads |= num << ((y / third * third + x / third) * self.size);
        tracing::debug!("rows={:b}", self.rows);
        tracing::debug!("cols={:b}", self.cols);
        tracing::debug!("quads={:b}", self.quads);
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.grid[self.index(x, y)]
    }

    fn can_be_placed(&self, x: usize, y: usize, num: u8) -> bool {
        let b = (y / 3) * 3 + (x / 3);
        let mask = (self.rows >> (y * 9)) | (self.cols >> (x * 9)) | (self.quads >> (b * 9));
        let mask_num = 1 << (num + 1);
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
        write!(f, "{}:{}:{}", self.size, self.seed, grid_line)
    }
}
