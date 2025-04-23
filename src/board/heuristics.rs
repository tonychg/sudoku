use itertools::Itertools;

fn in_column(grid: &[Vec<u8>], size: usize, x: usize, value: u8) -> bool {
    for y in 0..size {
        if grid[y][x] == value {
            return true;
        }
    }
    false
}

fn in_row(grid: &[Vec<u8>], size: usize, y: usize, value: u8) -> bool {
    for x in 0..size {
        if grid[y][x] == value {
            return true;
        }
    }
    false
}

fn in_square(grid: &[Vec<u8>], size: usize, x: usize, y: usize, value: u8) -> bool {
    let third = grid.len() / 3;
    let first_x = x / third * third;
    let first_y = y / third * third;

    for (x, y) in (0..size).map(|i| (first_x + i % third, first_y + i / third)) {
        if grid[y][x] == value {
            return true;
        }
    }
    false
}

pub(crate) fn can_be_placed(grid: &[Vec<u8>], size: usize, x: usize, y: usize, value: u8) -> bool {
    !in_column(grid, size, x, value)
        && !in_row(grid, size, y, value)
        && !in_square(grid, size, x, y, value)
}

pub(crate) fn is_valid(grid: &[Vec<u8>], size: usize) -> bool {
    let third = size / 3;

    for (x, y) in (0..size).into_iter().tuple_combinations() {
        let first_x = x / third * third;
        let first_y = y / third * third;
        if grid[y][x] == 0 {
            return false;
        }
        for i in 0..size {
            if x != i && grid[y][i] == grid[y][x] {
                return false;
            }
            if y != i && grid[i][x] == grid[y][x] {
                return false;
            }
            let square_x = first_x + i % third;
            let square_y = first_y + i / third;
            if x != square_x && y != square_y && grid[y][x] == grid[square_y][square_x] {
                return false;
            }
        }
    }
    true
}

pub(crate) fn next_empty_from_range(
    grid: &[Vec<u8>],
    x_range: &[usize],
    y_range: &[usize],
) -> Option<(usize, usize)> {
    for y in y_range.iter() {
        for x in x_range.iter() {
            if grid[*y][*x] == 0 {
                return Some((*x, *y));
            }
        }
    }
    None
}

pub(crate) fn next_empty_top_left(grid: &[Vec<u8>], size: usize) -> Option<(usize, usize)> {
    for y in 0..size {
        for x in 0..size {
            if grid[y][x] == 0 {
                return Some((x, y));
            }
        }
    }
    None
}
