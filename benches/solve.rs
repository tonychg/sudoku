use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use std::hint::black_box;
use sudoku::board::Board;
use sudoku::board::BoardBackend;
use sudoku::dfs::dfs;

fn solve_id(target: &str) -> usize {
    let parsed = Board::from_str(target, &BoardBackend::Grid).unwrap();
    dfs(
        vec![parsed],
        |b| b.id(),
        |b| b.completed(),
        |b| b.neighbors(),
    )
    .count()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve dfs id starting_numbers=26", |b| {
        b.iter(|| {
            solve_id(black_box("9:13461536571700027082:000006010500000080000870302000000009000960700605000103002007050000403000804600000"))
        })
    });
    c.bench_function("solve dfs id starting_numbers=26", |b| {
        b.iter(|| {
            solve_id(black_box("9:4498377688275560302:307400100006023800000000030000000008050069000020030001200000490000000006409106000"))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
