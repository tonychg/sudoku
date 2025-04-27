mod bitfield;
mod grid;

use std::fmt::Debug;
use std::fmt::Display;

use rand::Rng;
use uuid::Uuid;

use bitfield::BitFieldBoard;
use grid::GridBoard;

use crate::dfs::dfs;
use crate::dfs::dfs_with_max_depth;
use crate::rng;

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
        write!(
            f,
            "{}:{}:{}",
            self.inner.size(),
            self.inner.seed(),
            grid_line
        )
    }
}

impl Board {
    /// Serialize a board from string
    /// Format must be of the following form
    /// example: size:seed:grid
    ///     size 9
    ///     seed 12288288828
    ///     grid 00010202000..
    pub fn from_str(input: impl ToString, backend: &BoardBackend) -> anyhow::Result<Self> {
        let inner = GridBoard::from_str(input)?;
        let board = Self {
            id: Uuid::new_v4(),
            inner: inner.clone(),
            bit_fields: Self::new_bit_fields(inner.size(), backend),
        };
        Ok(board)
    }

    /// Backtrack all solutions
    /// If randomize is true, neighbors will be choosen randomly
    pub fn backtracking(&self, randomize: bool) -> impl Iterator<Item = Board> {
        dfs(
            vec![Self::from_board(self)],
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
            vec![Self::from_board(self)],
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
        dfs(
            vec![Self::from_board(self)],
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
        .enumerate()
        .take_while(|(i, _)| *i < limit)
        .count()
    }

    /// Create a new board from given parameter
    /// max_depth is used to avoid going too deep on each branches, this really
    /// speedup the process of generating a complete greed
    pub fn generate(
        size: usize,
        seed: Option<u64>,
        backend: BoardBackend,
        max_depth: usize,
    ) -> anyhow::Result<Self> {
        let board = Self::new(size, seed, &backend);
        tracing::debug!(
            size = board.inner.size(),
            seed = board.inner.seed(),
            "Generate a new board"
        );
        match board.backtracking_with_max_depth(max_depth, true).next() {
            Some(board) => Ok(board),
            None => Err(anyhow::anyhow!("Failed to generate board")),
        }
    }

    /// Make a board playable
    /// This is done by removing a number randomly until we reached the desired starting numbers.
    /// For each removed number we try to solve the board, if there is more than one solution
    /// the board is discarded
    pub fn make_playable(&self, starting_numbers: usize) -> Self {
        let mut holes = Vec::new();
        let mut rng = rng::rng_from_seed(self.inner.seed());
        let mut playable = Self::from_board(&self);
        let total = self.inner.size() * self.inner.size();
        while holes.len() < total - starting_numbers {
            let index = rng.random_range(0..total);
            let (x, y) = (index % self.inner.size(), index / self.inner.size());
            if playable.inner.get(x, y) != 0 {
                holes.push((x, y, playable.inner.get(x, y)));
                playable.inner.set(x, y, 0);
                if playable.count_solutions(2, true) != 1 {
                    if let Some((x, y, num)) = holes.pop() {
                        playable.inner.set(x, y, num);
                    }
                } else {
                    tracing::debug!("Current starting numbers {}", total - holes.len());
                }
            }
        }
        playable
    }

    /// Pretty print the grid
    pub fn to_pretty_grid(&self) -> String {
        let size = self.inner.size();
        let line = (0..size * 2 + 7)
            .map(|i| if i % 8 == 0 { "+" } else { "-" })
            .fold(String::new(), |acc, c| acc + c);
        let mut output = String::new();
        for y in 0..size {
            for x in 0..size {
                let num = self.inner.get(x, y);
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

    pub fn seed(&self) -> u64 {
        self.inner.seed()
    }

    fn new(size: usize, seed: Option<u64>, backend: &BoardBackend) -> Self {
        Self {
            id: Uuid::new_v4(),
            inner: GridBoard::new(
                size,
                match seed {
                    Some(seed) => seed,
                    None => rng::generate_seed(),
                },
            ),
            bit_fields: Self::new_bit_fields(size, backend),
        }
    }

    fn new_bit_fields(size: usize, backend: &BoardBackend) -> Option<BitFieldBoard> {
        match backend {
            BoardBackend::Grid => None,
            BoardBackend::BitField => Some(BitFieldBoard::new(size)),
        }
    }

    fn from_board(board: &Board) -> Self {
        Self {
            id: Uuid::new_v4(),
            inner: board.inner.clone(),
            bit_fields: board.bit_fields.clone(),
        }
    }

    fn id(&self) -> Uuid {
        self.id
    }

    fn completed(&self) -> bool {
        !self.inner.next_empty().is_some()
    }

    fn create_neighbors(&self, x: usize, y: usize) -> Vec<Self> {
        let mut neighbors = Vec::new();
        for num in 1..=9u8 {
            if self.inner.can_be_placed(x, y, num) {
                let mut next_board = Self::from_board(self);
                next_board.inner.set(x, y, num);
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
}
