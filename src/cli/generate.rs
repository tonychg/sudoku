use anyhow::Result;

use crate::grid::generate;
use crate::grid::make_playable;
use crate::grid::print_pretty_grid;
use crate::grid::print_raw_grid;
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
    let grid = generate(size, seed, max_iterations);
    let playable = make_playable(&grid, starting_numbers, seed);

    println!("seed: {}", seed);

    if raw {
        print_raw_grid(&grid);
        println!();
        print_raw_grid(&playable);
    } else {
        print_pretty_grid(&grid);
        println!();
        print_pretty_grid(&playable);
    }
    Ok(())
}
