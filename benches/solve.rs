use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use std::hint::black_box;
use sudoku::grid;

fn solve(target: &str) {
    let mut parsed = grid::parse(target);
    grid::solve(&mut parsed);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve starting_numbers=17", |b| {
        b.iter(|| {
            solve(black_box(
                r#"000004350
500009000
000000000
006730000
309000001
070090036
050000003
460002005
701000008"#,
            ))
        })
    });
    c.bench_function("solve starting_numbers=24", |b| {
        b.iter(|| {
            solve(black_box(
                r#"000000780
000970000
050000020
580007900
600000000
000608010
000000000
090000000
006000008"#,
            ))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
