use std::io;
use std::path::PathBuf;

use crate::board::Board;
use crate::board::BoardBackend;
use crate::file::read_boards;

#[derive(clap::Args, Clone, Debug)]
pub(crate) struct ShowArgs {
    /// Solve boards from file
    source: Option<PathBuf>,
    /// Read boards from stdin
    #[arg(short, long, default_value_t = false)]
    stdin: bool,
}

#[tracing::instrument]
pub fn cmd_show(args: &ShowArgs) -> anyhow::Result<()> {
    if let Some(source) = &args.source {
        for board in read_boards(&source.as_path())? {
            println!("{}", board.to_pretty_grid());
        }
    }
    if args.stdin {
        for line in io::stdin().lines().map_while(Result::ok) {
            let board = Board::from_str(line, &BoardBackend::default())?;
            println!("{}", board.to_pretty_grid());
        }
    }
    Ok(())
}
