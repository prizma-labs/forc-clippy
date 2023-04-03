#![cfg_attr(feature = "deny-warnings", deny(warnings))]
// warn on lints, that are included in `rust-lang/rust`s bootstrap
#![warn(rust_2018_idioms, unused_lifetimes)]

use std::env;
use std::process::{self};

const FORC_CLIPPY_HELP: &str = r#"Checks a package to catch common mistakes and improve your Rust code.

Usage:
    forc-clippy [options] [--] [<opts>...]

Common options:
    --no-deps                Run Clippy only on the given crate, without linting the dependencies
    -h, --help               Print this message
    -V, --version            Print version info and exit
    --explain LINT           Print the documentation for a given lint

To allow or deny a lint from the command line you can use `forc-clippy --`
with:

    -W --warn OPT       Set lint warnings
    -A --allow OPT      Set lint allowed
    -D --deny OPT       Set lint denied
    -F --forbid OPT     Set lint forbidden

You can use tool lints to allow or deny lints from your code, e.g.:

    #[allow(clippy::needless_lifetimes)]
"#;

fn show_help() {
    println!("{FORC_CLIPPY_HELP}");
}

fn show_version() {
    let version_info = rustc_tools_util::get_version_info!();
    println!("{version_info}");
}

pub fn main() {
    // Check for version and help flags even when invoked as 'cargo-clippy'
    if env::args().any(|a| a == "--help" || a == "-h") {
        show_help();
        return;
    }

    if env::args().any(|a| a == "--version" || a == "-V") {
        show_version();
        return;
    }

    if let Some(pos) = env::args().position(|a| a == "--explain") {
        if let Some(mut lint) = env::args().nth(pos + 1) {
            lint.make_ascii_lowercase();
            // clippy_lints::explain(&lint.strip_prefix("clippy::").unwrap_or(&lint).replace('-', "_"));
        } else {
            show_help();
        }
        return;
    }

    if let Err(code) = process(env::args().skip(2)) {
        process::exit(code);
    }
}

fn process<I>(_old_args: I) -> Result<(), i32>
    where
        I: Iterator<Item = String>,
{
    Ok(())
}
