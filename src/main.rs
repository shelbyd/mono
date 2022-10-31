//! Command line utilities for managing mono-repos.
//!
//! Meant to be used by developers and in CI/CD.
//!
//! ## Usage
//!
//! Any subdirectory can have a `MONO` file that specifies how that directory relates to the rest
//! of the project.
//!
//! ### Sync Files
//!
//! MONO files can specify files to sync from one location in the project to another.
//!
//! ```yaml
//! sync_files:
//!   - from: .github/workflows/*
//!     to: /.github/workflows/${ dir_slug }-${ file_name }
//! ```
//!
//! The above config, when located at, `foo/MONO` will copy a file like
//! `foo/.github/workflows/test.yml` to `.github/workflows/foo-test.yml`.

use std::error::Error;
use structopt::StructOpt;

mod commands;
mod interpolated_string;
mod monofile;

pub use interpolated_string::*;
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
#[structopt(about = "Manage mono-repos")]
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
    /// Ensure local directory is correct, changing files if necessary.
    Sync(commands::SyncCommand),
}

impl Command {
    fn run(self) -> Result<(), Box<dyn Error>> {
        match self {
            Command::Sync(cmd) => cmd.run(),
        }
    }
}
