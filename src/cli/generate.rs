use anyhow::Result;

use crate::grid;
use crate::rng;

pub fn generate_command(
    max_iterations: usize,
    seed: Option<u64>,
    size: usize,
    starting_numbers: usize,
    raw: bool,
) -> Result<()> {
    let seed = match seed {
        Some(seed) => seed,
        None => rng::generate_seed(),
    };
    let grid = grid::generate(size, seed, max_iterations);
    let playable = grid::make_playable(&grid, starting_numbers, seed);

    println!("seed: {}", seed);

    if raw {
        grid::print_raw(&grid);
        println!();
        grid::print_raw(&playable);
    } else {
        grid::print_pretty(&grid);
        println!();
        grid::print_pretty(&playable);
    }
    Ok(())
}
