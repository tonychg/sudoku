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
