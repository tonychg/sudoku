use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use std::hint::black_box;
use sudoku::board;

fn solve_recursive(target: &str) {
    let mut parsed = board::parse(target);
    let size = parsed.len();
    board::solve(&mut parsed, size);
}

fn solve_dfs(target: &str) {
    let parsed = board::parse(target);
    let size = parsed.len();
    let root = board::dfs::Node::new(&parsed, size);
    board::dfs::dfs(root, None);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("solve recursive starting_numbers=17", |b| {
        b.iter(|| {
            solve_recursive(black_box(
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
    c.bench_function("solve dfs starting_numbers=17", |b| {
        b.iter(|| {
            solve_dfs(black_box(
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
    c.bench_function("solve recursive starting_numbers=24", |b| {
        b.iter(|| {
            solve_recursive(black_box(
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
    c.bench_function("solve dfs starting_numbers=24", |b| {
        b.iter(|| {
            solve_dfs(black_box(
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
