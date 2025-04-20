use sudoku::cli::CliRunner;

fn main() -> anyhow::Result<()> {
    CliRunner::init().run()
}
