use super::super::{
    cli_opt::{
        DetailLevel::{self, *},
        LogFormat::{self, *},
    },
    cross_platform_path,
    diff::Diff,
    term::color::*,
};
use std::path::Path;

/// Log found filesystem object and maybe diff if `--details` is not `count`.
pub type Act<'a> = Box<dyn Fn(&Path, &str, &str) + 'a>;

/// Lookup a function that may log found filesystem object according to `--details`.
/// * If `--details=count`, the returning function would do nothing.
/// * If `--details=name`, the returning function would log names.
/// * If `--details=diff`, the returning function would log names and diffs.
pub fn get(details: DetailLevel, log_format: LogFormat, theme: &dyn ColorScheme) -> Act {
    let stringify_path = |path: &Path| cross_platform_path::to_string(path, '/');
    let format_name = move |path: &Path| {
        let message = format!("✗ {}", stringify_path(path));
        format!("{}", theme.diff().paint(message))
    };
    let print_name = move |path: &Path| {
        println!("{}", format_name(path));
    };
    match (details, log_format) {
        (Count, _) => Box::new(|_, _, _| ()),
        (Name, Human) => Box::new(move |path, _, _| print_name(path)),
        (Name, GitHubActions) => Box::new(move |path, _, _| {
            println!("::error file={}::Format error", stringify_path(path));
            print_name(path);
        }),
        (Diff, Human) => Box::new(move |path, old, new| {
            print_name(path);
            for line in Diff::new(old, new).lines(theme, ("   ", "  +", "  -")) {
                println!("{}", line);
            }
        }),
        (Diff, GitHubActions) => Box::new(move |path, old, new| {
            println!("::error file={}::Format error", stringify_path(path));
            println!("::group::{}", format_name(path));
            for line in Diff::new(old, new).lines(theme, (' ', '+', '-')) {
                println!("{}", line);
            }
            println!("::endgroup::");
        }),
    }
}
