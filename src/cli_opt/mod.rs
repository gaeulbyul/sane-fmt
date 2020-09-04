pub mod detail_level;
pub mod input_stream_address;
pub mod log_format;
pub mod when;

pub use detail_level::*;
pub use input_stream_address::*;
pub use log_format::*;
use std::{env::args, process::exit};
use structopt::*;
pub use when::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "sane-fmt", rename_all = "kebab")]
pub struct CliOpt {
    /// Reads unformatted code from standard input,
    /// prints formatted code to standard output, then exits.
    #[structopt(long)]
    pub stdio: bool,

    /// Whether to write or check
    #[structopt(long, short = "w")]
    pub write: bool,

    /// File diff detail
    /// [possible values: count, name, diff]
    #[structopt(long, default_value = "name")]
    pub details: DetailLevel,

    /// Do not log passed filenames
    #[structopt(long)]
    pub hide_passed: bool,

    /// When to use terminal color
    /// [possible values: auto, never, always]
    #[structopt(long, default_value = "auto")]
    pub color: When,

    /// Format of log messages
    /// [possible values: human, github-actions]
    #[structopt(long, default_value = "human")]
    pub log_format: LogFormat,

    /// Files whose contents contain paths to target files
    /// (`-` means stdin, other strings mean text file)
    #[structopt(long, short = "I", number_of_values = 1)]
    pub include: Vec<InputStreamAddress>,

    /// Files to process
    ///
    /// If none are provided, a default set of files will be assumed
    #[structopt(name = "files")]
    pub files: Vec<String>,
}

impl CliOpt {
    /// Parse arguments from `env::args`.
    ///
    /// Unlike `StructOpt::from_args`, this function treat unknown flags as errors.
    pub fn get() -> Self {
        match Self::from_iter_safe(args()) as Result<Self, clap::Error> {
            Ok(value) => value,
            Err(clap::Error { kind, message, .. }) => match kind {
                clap::ErrorKind::HelpDisplayed | clap::ErrorKind::VersionDisplayed => {
                    println!("{}", message);
                    exit(0);
                }
                _ => {
                    println!("{}", message);
                    exit(1);
                }
            },
        }
    }
}
