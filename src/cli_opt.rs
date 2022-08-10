pub mod detail_level;
pub mod input_stream_address;
pub mod log_format;
pub mod when;

pub use detail_level::*;
pub use input_stream_address::*;
pub use log_format::*;
pub use when::*;

use clap::Parser;

/// Opinionated code formatter for TypeScript and JavaScript
#[derive(Debug, Parser)]
#[clap(name = "sane-fmt", rename_all = "kebab", version)]
pub struct CliOpt {
    /// Reads unformatted code from standard input,
    /// prints formatted code to standard output, then exits
    #[clap(long)]
    pub stdio: bool,

    /// Whether to write or check
    #[clap(long, short = 'w')]
    pub write: bool,

    /// File diff detail
    #[clap(long, default_value = "name", possible_values = &["count", "name", "diff"])]
    pub details: DetailLevel,

    /// Do not log passed filenames
    #[clap(long)]
    pub hide_passed: bool,

    /// When to use terminal color
    #[clap(long, default_value = "auto", possible_values = &["auto", "never", "always"])]
    pub color: When,

    /// Format of log messages
    #[clap(long, default_value = "human", possible_values = &["human", "github-actions"])]
    pub log_format: LogFormat,

    /// Files whose contents contain paths to target files
    /// (`-` means stdin, other strings mean text file)
    #[clap(long, short = 'I')]
    pub include: Option<InputStreamAddress>,

    /// Files to process
    ///
    /// If none are provided, a default set of files will be assumed
    pub files: Vec<String>,
}
