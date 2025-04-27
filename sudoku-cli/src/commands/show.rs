use std::path::PathBuf;

use crate::file::list_boards;

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
    for board in list_boards(args.source.as_ref(), args.stdin)? {
        println!("{}", board.to_pretty_grid());
    }
    Ok(())
}
