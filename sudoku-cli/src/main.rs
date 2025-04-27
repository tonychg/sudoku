use sudoku_cli::CliRunner;

fn main() -> anyhow::Result<()> {
    CliRunner::init().run()
}
