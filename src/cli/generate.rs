use anyhow::Result;

use crate::board::BoardGenerator;

pub fn generate(
    max_iterations: usize,
    seed: Option<u64>,
    size: usize,
    starting_numbers: usize,
) -> Result<()> {
    let mut generator = BoardGenerator::builder()
        .max_iterations(max_iterations)
        .size(size)
        .build();
    if let Some(seed) = seed {
        generator.board.seed = seed;
    }
    let board = generator.board();
    let playable = generator.make_playable(size * size - starting_numbers);

    println!("seed: {}", board.seed);
    println!("{}", board);
    println!("{}", playable.board);
    Ok(())
}
