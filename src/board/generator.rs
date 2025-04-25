use super::Board;
use crate::dfs::solve_dfs;
use crate::rng;
use anyhow::Result;
use rand::Rng;

pub struct BoardGenerator {
    seed: u64,
    size: usize,
    starting_numbers: usize,
    max_iterations: usize,
}

impl BoardGenerator {
    pub fn new(
        size: usize,
        seed: Option<u64>,
        starting_numbers: usize,
        max_iterations: usize,
    ) -> Self {
        Self {
            seed: match seed {
                Some(seed) => seed,
                None => rng::generate_seed(),
            },
            starting_numbers,
            size,
            max_iterations,
        }
    }

    pub fn make_playable<T: Clone + Board>(&self, board: &T) -> T {
        let mut holes = Vec::new();
        let mut rng = rng::rng_from_seed(self.seed);
        let mut playable = board.clone();
        let total = self.size * self.size;

        while holes.len() < total - self.starting_numbers {
            let index = rng.random_range(0..total);
            let (x, y) = (index % self.size, index / self.size);
            if playable.get(x, y) == 0 {
                continue;
            }
            holes.push((x, y, playable.get(x, y)));
            playable.set(x, y, 0);
            let next = playable.clone();
            let solutions = solve_dfs(next, None, Some(2), None);
            if solutions.len() != 1 {
                if let Some((x, y, num)) = holes.pop() {
                    playable.set(x, y, num);
                }
            }
        }

        playable
    }

    pub fn generate<T: Clone + Board>(&self) -> Result<T> {
        let empty = T::new(self.size);
        let boards = solve_dfs(empty, Some(self.seed), Some(1), Some(self.max_iterations));
        tracing::info!("Complete board generated with seed {}", self.seed);
        if boards.is_empty() {
            return Err(anyhow::anyhow!("Failed to generate board"));
        }
        Ok(boards[0].clone())
    }
}
