use rand::seq::SliceRandom;

use crate::rng;

#[derive(Debug, Clone)]
pub struct GridBoard {
    size: usize,
    pub grid: Vec<u8>,
    seed: u64,
    x_range: Vec<usize>,
    y_range: Vec<usize>,
}

impl GridBoard {
    pub fn new(size: usize, seed: u64) -> Self {
        let mut y_range = (0..size).collect::<Vec<usize>>();
        let mut x_range = y_range.clone();
        let mut rng = rng::rng_from_seed(seed);

        x_range.shuffle(&mut rng);
        y_range.shuffle(&mut rng);

        Self {
            size,
            seed,
            grid: vec![0; size * size],
            y_range,
            x_range,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, num: u8) {
        let index = self.index(x, y);
        self.grid[index] = num;
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.grid[self.index(x, y)]
    }

    pub fn can_be_placed(&self, x: usize, y: usize, num: u8) -> bool {
        !self.in_column(x, num) && !self.in_row(y, num) && !self.in_quad(x, y, num)
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn next_empty(&self) -> Option<(usize, usize)> {
        for (index, num) in self.grid.iter().enumerate() {
            if *num == 0 {
                return Some(self.xy(index));
            }
        }
        None
    }

    pub fn next_empty_random(&self) -> Option<(usize, usize)> {
        for y in self.y_range.iter() {
            for x in self.x_range.iter() {
                let index = self.index(*x, *y);
                if self.grid[index] == 0 {
                    return Some((*x, *y));
                }
            }
        }
        None
    }

    pub fn xy(&self, index: usize) -> (usize, usize) {
        (index % self.size, index / self.size)
    }

    fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.size
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
