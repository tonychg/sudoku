use std::hint::black_box;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;

use sudoku::board::Board;
use sudoku::board::BoardBackend;
use sudoku::dfs::dfs;

fn bench_linear_solve(c: &mut Criterion, target: &str) {
    c.bench_function(
        &format!("solve dfs id starting_numbers=26 grid={}", target),
        |b| {
            b.iter(|| {
                let parsed = Board::from_str(target, &BoardBackend::Grid).unwrap();
                dfs(
                    vec![parsed],
                    |b| b.id(),
                    |b| b.completed(),
                    |b| b.neighbors(),
                )
                .count()
            })
        },
    );
}

fn criterion_benchmark(c: &mut Criterion) {
    vec![
        "9:13461536571700027082:000006010500000080000870302000000009000960700605000103002007050000403000804600000",
        "9:4498377688275560302:307400100006023800000000030000000008050069000020030001200000490000000006409106000",
    ].iter().for_each(|board| {
        bench_linear_solve(c, black_box( board))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
