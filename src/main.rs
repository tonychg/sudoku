use sudoku::board::BoardGenerator;

fn main() {
    let mut generator = BoardGenerator::builder()
        .max_iterations(1000)
        // .seed(4551893143925369887)
        .build();
    let board = generator.board();

    println!("seed: {}", board.seed);
    println!("{}", board);
}
