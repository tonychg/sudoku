struct BoardGenerator {
    seed: u64,
    size: usize,
}

impl BoardGenerator {
    pub fn new(seed: u64, size: usize) -> Self {
        Self { seed, size }
    }
}
