use anyhow::Result;

use crate::board::BoardGenerator;

pub fn generate(max_iterations: usize, seed: Option<u64>, size: usize) -> Result<()> {
    let mut generator = BoardGenerator::builder()
        .max_iterations(max_iterations)
        .size(size)
        .build();
    if let Some(seed) = seed {
        generator.board.seed = seed;
    }
    let board = generator.board();

    println!("seed: {}", board.seed);
    println!("{}", board);
    Ok(())
}
