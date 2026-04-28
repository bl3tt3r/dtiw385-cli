use crate::cli::*;
use clap::Parser;
use std::io::{self, IsTerminal};
use tokio::io::{AsyncBufReadExt, BufReader};

mod cli;
mod errors;

/// Entry point of the `dtiw385` CLI.
///
/// Parses the command from arguments, then runs it either once (terminal mode)
/// or for each JSON line received from stdin (pipe mode).
///
/// # Exit codes
///
/// - `0` — all inputs processed successfully
/// - `1` — partial success (some inputs failed)
/// - `2` — fatal error (no input read, or all inputs failed)
#[tokio::main]
async fn main() {
    let mut input = 0;
    let mut errors = 0;

    let cli = Cli::parse();
    let command = cli.command;

    if io::stdin().is_terminal() {
        input += 1;
        if !command.execute().await {
            errors += 1;
        }
    } else {
        let stdin = tokio::io::stdin();
        let mut lines = BufReader::new(stdin).lines();
        while let Ok(Some(json)) = lines.next_line().await {
            input += 1;
            let mut command = command.clone(); // snapshot of the initial command
            if !command.merge_from_json(&json) || !command.execute().await {
                errors += 1;
            }
        }
    }

    std::process::exit(match (input, errors) {
        (0, _) => 2,          // nothing read → fatal error
        (_, 0) => 0,          // all good
        (i, e) if e < i => 1, // partial success
        _ => 2,               // all failed → fatal
    });
}
