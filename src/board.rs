mod bitfield;
mod generator;
mod grid;

pub use bitfield::BitFieldBoard;
pub use generator::BoardGenerator;
pub use grid::GridBoard;
use std::fmt::Debug;

pub trait Board: Send + Sync + Debug {
    fn new(size: usize) -> Self;
    fn size(&self) -> usize;
    fn set(&mut self, x: usize, y: usize, num: u8);
    fn get(&self, x: usize, y: usize) -> u8;
    fn next_empty(&self) -> Option<(usize, usize)>;
    fn next_empty_random(&self, y_range: &[usize], x_range: &[usize]) -> Option<(usize, usize)>;
    fn can_be_placed(&self, x: usize, y: usize, num: u8) -> bool;
}

pub fn parse_grid_string(sgrid: impl ToString) -> anyhow::Result<(usize, String)> {
    let sgrid = sgrid.to_string();
    let data = sgrid.split(":").collect::<Vec<&str>>();
    if data.len() != 2 {
        return Err(anyhow::anyhow!("Invalid format size:grid"));
    }
    let size: usize = data[0].parse()?;
    let grid = data[1].to_string();
    Ok((size, grid))
}

pub fn to_pretty_grid(board: &impl Board) -> String {
    let size = board.size();
    let line = (0..size * 2 + 7)
        .map(|i| if i % 8 == 0 { "+" } else { "-" })
        .fold(String::new(), |acc, c| acc + c);
    let mut output = String::new();
    for y in 0..size {
        for x in 0..size {
            let num = board.get(x, y);
            let num_char = if num == 0 { " " } else { &format!("{}", num) };
            let elem = if y == 0 && x == 0 {
                format!("{}\n| {}", line, num_char)
            } else if !(x != size - 1 || y <= 1 && y != size - 1 || y % 3 != 2 && y != size - 1) {
                format!(" {} |\n{}\n", num_char, line)
            } else if x > 1 && (x - 1) % 3 == 2 {
                format!(" | {}", num_char)
            } else if x == size - 1 {
                format!(" {} |\n", num_char)
            } else if x == 0 {
                format!("| {}", num_char)
            } else {
                format!(" {}", num_char)
            };
            output.push_str(&elem);
        }
    }
    output
}
