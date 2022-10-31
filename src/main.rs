use std::error::Error;
use structopt::StructOpt;

mod commands;
mod monofile;

pub use monofile::*;

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Options::from_args();

    stderrlog::new()
        .module(module_path!())
        .verbosity(opts.verbose + 1)
        .quiet(opts.quiet)
        .init()?;

    opts.command.run()?;
    Ok(())
}

#[derive(StructOpt)]
struct Options {
    /// Silence all output.
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,

    /// Verbose mode (-v, -vv, -vvv, etc).
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,

    /// Command to run.
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt)]
enum Command {
    Sync(commands::SyncCommand),
}

impl Command {
    fn run(self) -> Result<(), Box<dyn Error>> {
        match self {
            Command::Sync(cmd) => cmd.run(),
        }
    }
}
