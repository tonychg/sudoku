use std::io;

use crate::board::Board;
use crate::board::BoardBackend;

#[derive(clap::Args, Clone, Debug)]
pub(crate) struct ShowArgs {
    /// Read board from stdin
    #[arg(short, long, default_value_t = true)]
    stdin: bool,
}

#[tracing::instrument]
pub fn cmd_show(args: &ShowArgs) -> anyhow::Result<()> {
    if args.stdin {
        for line in io::stdin().lines().map_while(Result::ok) {
            let board = Board::from_str(line, &BoardBackend::default())?;
            println!("{}", board.to_pretty_grid());
        }
    }
    Ok(())
}
