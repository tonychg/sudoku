use std::fmt::Display;

use rand::Rng;

use crate::Grid;
use crate::SIZE;
use crate::dfs::dfs;
use crate::dfs::dfs_with_max_depth;
use crate::rng;

#[derive(Debug, Clone)]
pub struct Board {
    seed: u64,
    inner: Grid,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid_line = self
            .inner
            .digits
            .iter()
            .map(|num| format!("{}", num))
            .collect::<Vec<String>>()
            .join("");
        write!(f, "{}:{}:{}", SIZE, self.seed, grid_line)
    }
}

impl Board {
    /// Serialize a board from string
    /// Format must be of the following form
    /// example: size:seed:grid
    ///     size 9
    ///     seed 12288288828
    ///     grid 00010202000..
    pub fn from_str(input: impl ToString) -> anyhow::Result<Self> {
        let inner = Grid::from_str(input)?;
        let board = Self {
            inner: inner.clone(),
            seed: inner.seed,
        };
        Ok(board)
    }

    /// Backtrack all solutions
    /// If randomize is true, neighbors will be choosen randomly
    pub fn backtracking(&self, randomize: bool) -> impl Iterator<Item = Board> {
        let root = vec![self.inner.clone()];
        dfs(
            root,
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
        .map(|g| Self {
            seed: self.seed,
            inner: g,
        })
    }

    /// Backtrack all solutions
    /// If randomize is true, neighbors will be choosen randomly
    /// Maximum depth parameter specify when branch will be cut
    pub fn backtracking_with_max_depth(
        &self,
        max_depth: usize,
        randomize: bool,
    ) -> impl Iterator<Item = Board> {
        let root = vec![self.inner.clone()];
        dfs_with_max_depth(
            root,
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
        .map(|g| Self {
            seed: self.seed,
            inner: g,
        })
    }

    /// Traversing in DFS order the solutions graph
    /// If limit is reached break the loop an return the limit
    pub fn count_solutions(&self, limit: usize, randomize: bool) -> usize {
        let root = self.inner.clone();
        dfs(
            vec![root],
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
    pub fn generate(seed: Option<u64>, max_depth: usize) -> anyhow::Result<Self> {
        let board = Self::new(seed);
        tracing::debug!("Generate a new board");
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
        let mut rng = rng::rng_from_seed(self.seed);
        let mut playable = self.clone();
        let total = SIZE * SIZE;
        while holes.len() < total - starting_numbers {
            let index = rng.random_range(0..total);
            let (x, y) = (index % SIZE, index / SIZE);
            if playable.inner.get_digit(x, y) != 0 {
                holes.push((x, y, playable.inner.get_digit(x, y)));
                playable.inner.set_digit(x, y, 0);
                if playable.count_solutions(2, true) != 1 {
                    if let Some((x, y, num)) = holes.pop() {
                        playable.inner.set_digit(x, y, num);
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
        self.inner.to_pretty_string()
    }

    pub fn seed(&self) -> u64 {
        self.inner.seed
    }

    fn new(seed: Option<u64>) -> Self {
        let seed = match seed {
            Some(seed) => seed,
            None => rng::generate_seed(),
        };
        Self {
            inner: Grid::new(seed),
            seed,
        }
    }
}
