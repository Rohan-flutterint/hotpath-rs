mod cmd;
use clap::{Parser, Subcommand};
use cmd::console::ConsoleArgs;
use eyre::Result;

#[cfg(feature = "tui")]
#[derive(Subcommand, Debug)]
pub enum HPSubcommand {
    #[command(about = "Launch TUI console to monitor profiling metrics in real-time")]
    Console(ConsoleArgs),
}

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "hotpath CLI: automatically profile Rust programs on each Pull Request

https://github.com/pawurb/hotpath"
)]
pub struct HPArgs {
    #[command(subcommand)]
    pub cmd: HPSubcommand,
}

#[hotpath::main(limit = 20)]
fn main() -> Result<()> {
    let root_args = HPArgs::parse();

    match root_args.cmd {
        HPSubcommand::Console(args) => {
            args.run()?;
        }
    }

    Ok(())
}
