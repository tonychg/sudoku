use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::board::Board;
use crate::board::BoardBackend;

fn create_board_path(destination: &Path, board: &Board) -> anyhow::Result<PathBuf> {
    if !destination.exists() || (!destination.is_file() && !destination.is_dir()) {
        anyhow::bail!("Destination not found");
    }
    let board_path = if destination.is_file() {
        destination.to_path_buf()
    } else {
        let filename = format!("{}.txt", board.seed());
        destination.join(filename)
    };
    Ok(board_path)
}

/// Write gnereated board to file
/// If destination path is a directory auto-generate filename
/// If destination path is a file append the board at the end
pub fn write_board(destination: &Path, board: &Board) -> anyhow::Result<()> {
    let board_path = create_board_path(destination, board)?;
    let mut board_file = if destination.is_dir() {
        File::open(board_path)?
    } else {
        File::options().append(true).open(board_path)?
    };
    writeln!(&mut board_file, "{}", board.to_string())?;
    Ok(())
}

/// Read boards from file
pub fn read_boards(source: &Path) -> anyhow::Result<Vec<Board>> {
    let file = File::open(source)?;
    let reader = BufReader::new(file);
    let mut boards = Vec::new();
    for line in reader.lines().map_while(Result::ok) {
        let board = Board::from_str(line, &BoardBackend::Grid)?;
        boards.push(board);
    }
    Ok(boards)
}
