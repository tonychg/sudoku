use anyhow::Result;
use rand::Rng;
use rand::seq::SliceRandom;
use uuid::Uuid;

use crate::board::Board;
use crate::board::BoardBackend;
use crate::dfs::dfs;
use crate::dfs::dfs_with_max_depth;
use crate::rng;

pub struct PlayableBoard {
    id: Uuid,
    size: usize,
    board: Board,
    holes: Vec<(usize, usize, u8)>,
    total: usize,
    x_range: Vec<usize>,
    y_range: Vec<usize>,
}

impl PlayableBoard {
    pub fn new(
        board: &Board,
        holes: Vec<(usize, usize, u8)>,
        x_range: Vec<usize>,
        y_range: Vec<usize>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            size: board.size(),
            board: Board::from_board(board),
            holes,
            total: board.size() * board.size(),
            x_range,
            y_range,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn starting_numbers(&self) -> usize {
        let n = self.total - self.holes.len();
        tracing::debug!("Starting numbers {}", n);
        n
    }

    pub fn neighbors(&self) -> Vec<Self> {
        let mut neighbors = vec![];
        for y in 0..self.size {
            for x in 0..self.size {
                if self.board.get(x, y) != 0 {
                    let hole = (x, y, self.board.get(x, y));
                    let mut playable = Board::from_board(&self.board);
                    playable.set(x, y, 0);
                    let next = Board::from_board(&playable);
                    let mut counter = 0;
                    for _ in dfs(vec![next], |b| b.id(), |b| b.completed(), |b| b.neighbors()) {
                        counter += 1;
                        if counter == 2 {
                            break;
                        }
                    }
                    if counter == 1 {
                        let mut holes = Vec::from(self.holes.clone());
                        holes.push(hole);
                        let neighbor =
                            Self::new(&playable, holes, self.x_range.clone(), self.y_range.clone());
                        neighbors.push(neighbor)
                    }
                }
            }
        }
        neighbors
    }
}

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

    #[allow(dead_code)]
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
            let mut counter = 0;
            for _ in dfs(
                vec![next],
                |b| b.id(),
                |b| b.completed(),
                |b| b.random_neighbors(),
            ) {
                counter += 1;
                if counter == 2 {
                    tracing::debug!(counter, "Found solution");
                    break;
                }
            }
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
        let mut rng = rng::rng_from_seed(board.seed());
        let mut x_range = (0..board.size()).collect::<Vec<usize>>();
        let mut y_range = x_range.clone();
        x_range.shuffle(&mut rng);
        y_range.shuffle(&mut rng);
        let root = PlayableBoard::new(board, vec![], x_range, y_range);
        for playable in dfs(
            vec![root],
            |p| p.id(),
            |p| p.starting_numbers() == self.starting_numbers,
            |p| p.neighbors(),
        ) {
            return Some(playable.board);
        }
        None
    }

    pub fn generate(&self, backend: &BoardBackend) -> Result<Board> {
        let empty = Board::new(self.size, self.seed, backend);
        tracing::debug!(size = self.size, seed = self.seed, "Generate a new board");
        for (index, board) in dfs_with_max_depth(
            vec![empty],
            |b| b.id(),
            |b| b.completed(),
            |b| b.random_neighbors(),
            self.max_depth,
        )
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
