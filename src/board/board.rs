#[derive(Debug, Clone)]
pub struct Board {
    size: usize,
    grid: Vec<u8>,
    rows: u128,
    cols: u128,
    quads: u128,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            rows: 0,
            cols: 0,
            quads: 0,
            grid: vec![0; size * size],
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.size
    }

    fn xy(&self, index: usize) -> (usize, usize) {
        (index % self.size, index / self.size)
    }

    pub fn next_empty(&self) -> Option<(usize, usize)> {
        for (index, num) in self.grid.iter().enumerate() {
            if *num == 0 {
                return Some(self.xy(index));
            }
        }
        None
    }

    pub fn set(&mut self, x: usize, y: usize, num: u8) {
        let third = self.size / 3;
        let index = self.index(x, y);
        self.rows |= 1 << (x * self.size + num as usize);
        self.cols |= 1 << (y * self.size + num as usize);
        self.quads |= 1 << ((y / third * third + x / third) * self.size + num as usize);
        self.grid[index] = num;
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.grid[self.index(x, y)]
    }

    pub fn can_be_placed(&self, x: usize, y: usize, num: u8) -> bool {
        let b = (y / 3) * 3 + (x / 3);
        let mask = (self.rows >> (y * 9)) | (self.cols >> (x * 9)) | (self.quads >> (b * 9));
        let mask_num = 1 << num;
        (mask & mask_num) == 0
    }
}
