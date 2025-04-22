use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use std::hint::black_box;
use sudoku::board::Board;
use sudoku::board::BoardGenerator;

fn solve_generated_board(seed: u64) -> Board {}
