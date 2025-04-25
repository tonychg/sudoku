use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use std::hint::black_box;
use sudoku::board::generate;

fn generate_boards(seed: u64, s: usize, max_i: usize) -> Vec<Vec<u8>> {
    generate(s, seed, max_i)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        "generate_boards seed=9693633096020010538 s=9 max_i=500",
        |b| {
            b.iter(|| generate_boards(black_box(9693633096020010538), black_box(9), black_box(500)))
        },
    );
    c.bench_function(
        "generate_boards seed=6564811992589904777 s=9 max_i=500",
        |b| {
            b.iter(|| generate_boards(black_box(6564811992589904777), black_box(9), black_box(500)))
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
