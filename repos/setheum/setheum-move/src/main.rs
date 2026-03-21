use setheum_move::setheum_move_cli;

fn main() -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;

    setheum_move_cli(cwd)
}
