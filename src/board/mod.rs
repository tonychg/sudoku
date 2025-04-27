mod bitfield;
mod grid;

use std::fmt::Debug;
use std::fmt::Display;

use uuid::Uuid;

use bitfield::BitFieldBoard;
use grid::GridBoard;

use crate::dfs::dfs;
use crate::dfs::dfs_with_max_depth;

#[derive(clap::ValueEnum, Default, Clone, Debug)]
pub enum BoardBackend {
    #[default]
    Grid,
    BitField,
}

pub struct Board {
    id: Uuid,
    inner: GridBoard,
    bit_fields: Option<BitFieldBoard>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid_line = self
            .inner
            .grid
            .iter()
            .map(|num| format!("{}", num))
            .collect::<Vec<String>>()
            .join("");
        write!(f, "{}:{}:{}", self.size(), self.seed(), grid_line)
    }
}

impl Board {
    pub fn new(size: usize, seed: u64, backend: &BoardBackend) -> Self {
        Self {
            id: Uuid::new_v4(),
            inner: GridBoard::new(size, seed),
            bit_fields: match backend {
                BoardBackend::Grid => None,
                BoardBackend::BitField => Some(BitFieldBoard::new(size)),
            },
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn size(&self) -> usize {
        self.inner.size()
    }

    pub fn seed(&self) -> u64 {
        self.inner.seed()
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.inner.set_seed(seed);
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.inner.get(x, y)
    }

    pub fn set(&mut self, x: usize, y: usize, num: u8) {
        self.inner.set(x, y, num);
        // if let Some(bit_fields) = &mut self.bit_fields {
        //     bit_fields.set(x, y, num);
        // }
    }

    pub fn from_board(board: &Board) -> Self {
        Self {
            id: Uuid::new_v4(),
            inner: board.inner.clone(),
            bit_fields: board.bit_fields.clone(),
        }
    }

    pub fn from_str(string_board: impl ToString, backend: &BoardBackend) -> anyhow::Result<Self> {
        let (size, seed, grid) = parse_grid_string(string_board)?;
        let mut board = Self::new(size, seed, backend);
        for (index, num) in grid.chars().enumerate() {
            let (x, y) = board.inner.xy(index);
            let num: u8 = num.to_string().parse()?;
            board.set(x, y, num);
        }
        Ok(board)
    }

    pub fn to_pretty_grid(&self) -> String {
        let size = self.size();
        let line = (0..size * 2 + 7)
            .map(|i| if i % 8 == 0 { "+" } else { "-" })
            .fold(String::new(), |acc, c| acc + c);
        let mut output = String::new();
        for y in 0..size {
            for x in 0..size {
                let num = self.get(x, y);
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

    fn completed(&self) -> bool {
        !self.inner.next_empty().is_some()
    }

    fn create_neighbors(&self, x: usize, y: usize) -> Vec<Self> {
        let mut neighbors = Vec::new();
        for num in 1..=9u8 {
            if self.inner.can_be_placed(x, y, num) {
                let mut next_board = Self::from_board(self);
                next_board.set(x, y, num);
                neighbors.push(next_board);
            }
        }
        neighbors
    }

    fn random_neighbors(&self) -> Vec<Self> {
        let mut neighbors = Vec::new();
        if let Some((x, y)) = self.inner.next_empty_random() {
            neighbors = self.create_neighbors(x, y);
        }
        neighbors
    }

    fn neighbors(&self) -> Vec<Self> {
        let mut neighbors = Vec::new();
        if let Some((x, y)) = self.inner.next_empty() {
            neighbors = self.create_neighbors(x, y);
        }
        neighbors
    }

    /// Backtrack all solutions
    /// If randomize is true, neighbors will be choosen randomly
    pub fn backtracking(&self, randomize: bool) -> impl Iterator<Item = Board> {
        dfs(
            vec![Board::from_board(self)],
            |b| b.id(),
            |b| b.completed(),
            move |b| {
                if randomize {
                    b.random_neighbors()
                } else {
                    b.neighbors()
                }
            },
        )
    }

    /// Backtrack all solutions
    /// If randomize is true, neighbors will be choosen randomly
    /// Maximum depth parameter specify when branch will be cut
    pub fn backtracking_with_max_depth(
        &self,
        max_depth: usize,
        randomize: bool,
    ) -> impl Iterator<Item = Board> {
        dfs_with_max_depth(
            vec![Board::from_board(self)],
            |b| b.id(),
            |b| b.completed(),
            move |b| {
                if randomize {
                    b.random_neighbors()
                } else {
                    b.neighbors()
                }
            },
            max_depth,
        )
    }

    /// Traversing in DFS order the solutions graph
    /// If limit is reached break the loop an return the limit
    pub fn count_solutions(&self, limit: usize, randomize: bool) -> usize {
        let mut counter = 0;
        for _ in dfs(
            vec![Board::from_board(self)],
            |b| b.id(),
            |b| b.completed(),
            move |b| {
                if randomize {
                    b.random_neighbors()
                } else {
                    b.neighbors()
                }
            },
        ) {
            counter += 1;
            if counter == limit {
                break;
            }
        }
        counter
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
