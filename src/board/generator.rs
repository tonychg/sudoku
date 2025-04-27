use anyhow::Result;
use rand::Rng;

use crate::board::Board;
use crate::board::BoardBackend;
use crate::rng;

pub struct BoardGenerator {
    seed: u64,
    size: usize,
}

impl BoardGenerator {
    pub fn new(size: usize, seed: Option<u64>) -> Self {
        Self {
            seed: match seed {
                Some(seed) => seed,
                None => rng::generate_seed(),
            },
            size,
        }
    }

    fn iterative_playable(&self, board: &Board, starting_numbers: usize) -> Board {
        let mut holes = Vec::new();
        let mut rng = rng::rng_from_seed(board.seed());
        let mut playable = Board::from_board(&board);
        let total = self.size * self.size;
        while holes.len() < total - starting_numbers {
            let index = rng.random_range(0..total);
            let (x, y) = (index % self.size, index / self.size);
            if playable.get(x, y) != 0 {
                holes.push((x, y, playable.get(x, y)));
                playable.set(x, y, 0);
                if playable.count_solutions(2, true) != 1 {
                    if let Some((x, y, num)) = holes.pop() {
                        playable.set(x, y, num);
                    }
                } else {
                    tracing::debug!("Current starting numbers {}", total - holes.len());
                }
            }
        }
        playable
    }

    pub fn make_playable(&self, board: &Board, starting_numbers: usize) -> Board {
        self.iterative_playable(board, starting_numbers)
    }

    pub fn generate(&self, backend: &BoardBackend, max_depth: usize) -> Result<Board> {
        let empty = Board::new(self.size, self.seed, backend);
        tracing::debug!(size = self.size, seed = self.seed, "Generate a new board");
        match empty.backtracking_with_max_depth(max_depth, true).next() {
            Some(board) => Ok(board),
            None => Err(anyhow::anyhow!("Failed to generate board")),
        }
    }
}
