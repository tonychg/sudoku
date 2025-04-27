use std::hint::black_box;

use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;

use sudoku::Board;

fn bench_linear_solve(c: &mut Criterion, target: &str) {
    c.bench_function(
        &format!("solve dfs id starting_numbers=26 grid={}", target),
        |b| {
            b.iter(|| {
                let parsed = Board::from_str(target).unwrap();
                parsed.backtracking(false).count()
            })
        },
    );
}

fn criterion_benchmark(c: &mut Criterion) {
    vec![
        "9:13461536571700027082:000006010500000080000870302000000009000960700605000103002007050000403000804600000",
        "9:4498377688275560302:307400100006023800000000030000000008050069000020030001200000490000000006409106000",
        "9:3942065167592087234:400000006603908000000006750080000090006009025000007000050000000008000001040761002",
        "9:3684658125043561604:006908000700006000050000008070590400000010072105800000009725600000000190000000000",
        "9:17007719621643639168:000100060020600000470000000000540002700001003902067000200000900000300001060052030",
    ].iter().for_each(|board| {
        bench_linear_solve(c, black_box( board))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
