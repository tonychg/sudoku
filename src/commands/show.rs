use crate::board::{GridBoard, to_pretty_grid};
use std::io;

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
            let board = GridBoard::from_str(line)?;
            println!("{}", to_pretty_grid(&board));
        }
    }
    Ok(())
}
