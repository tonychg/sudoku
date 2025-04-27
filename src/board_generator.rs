use anyhow::Result;
use rand::Rng;

use crate::board::Board;
use crate::board::BoardBackend;
use crate::rng;

pub struct BoardGenerator {
    seed: u64,
    size: usize,
    starting_numbers: usize,
    max_depth: usize,
}

impl BoardGenerator {
    pub fn new(size: usize, seed: Option<u64>, starting_numbers: usize, max_depth: usize) -> Self {
        Self {
            seed: match seed {
                Some(seed) => seed,
                None => rng::generate_seed(),
            },
            starting_numbers,
            size,
            max_depth,
        }
    }

    fn iterative_playable(&self, board: &Board) -> Board {
        let mut holes = Vec::new();
        let mut rng = rng::rng_from_seed(self.seed);
        let mut playable = Board::from_board(&board);
        let total = self.size * self.size;
        while holes.len() < total - self.starting_numbers {
            let index = rng.random_range(0..total);
            let (x, y) = (index % self.size, index / self.size);
            if playable.get(x, y) == 0 {
                continue;
            }
            holes.push((x, y, playable.get(x, y)));
            playable.set(x, y, 0);
            let next = Board::from_board(&playable);
            let counter = next.count_solutions(2, true);
            let current_starting_numbers = total - holes.len();
            if counter != 1 {
                if let Some((x, y, num)) = holes.pop() {
                    playable.set(x, y, num);
                }
            } else {
                tracing::debug!("Current starting numbers {}", current_starting_numbers);
            }
        }
        playable
    }

    pub fn make_playable(&self, board: &Board) -> Option<Board> {
        Some(self.iterative_playable(board))
    }

    pub fn generate(&self, backend: &BoardBackend) -> Result<Board> {
        let empty = Board::new(self.size, self.seed, backend);
        tracing::debug!(size = self.size, seed = self.seed, "Generate a new board");
        for (index, board) in empty
            .backtracking_with_max_depth(self.max_depth, true)
            .enumerate()
        {
            if index == 0 {
                tracing::debug!("Complete board generated");
                return Ok(board);
            }
        }
        Err(anyhow::anyhow!("Failed to generate board"))
    }
}
