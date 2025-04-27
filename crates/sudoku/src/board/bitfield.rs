/// TODO: not currently supported
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BitFieldBoard {
    size: usize,
    rows: u128,
    cols: u128,
    quads: u128,
}

#[allow(dead_code)]
impl BitFieldBoard {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            rows: 0,
            cols: 0,
            quads: 0,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, num: u8) {
        let third = self.size / 3;
        let num = 1 << (num + 1) as u128;
        self.rows |= num << (y * self.size);
        self.cols |= num << (x * self.size);
        self.quads |= num << ((y / third * third + x / third) * self.size);
    }

    pub fn can_be_placed(&self, x: usize, y: usize, num: u8) -> bool {
        let b = (y / 3) * 3 + (x / 3);
        let mask = (self.rows >> (y * 9)) | (self.cols >> (x * 9)) | (self.quads >> (b * 9));
        let mask_num = 1 << (num + 1);
        (mask & mask_num) == 0
    }
}
