use crate::errors::Error;
use clap::{CommandFactory, Parser, Subcommand, error::ErrorKind};
pub use command::*;
pub use parser::*;
use serde::Serialize;

pub mod command;
pub mod parser;

/// Root CLI structure parsed from command-line arguments.
#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands.
#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Scan a range of IPs and ports to discover decoders.
    Scan(Scan),
    /// Retrieve information from a specific decoder.
    Infos(Infos),
}

impl Commands {
    /// Merges fields from a JSON string into the current command.
    ///
    /// Returns `true` on success, `false` if parsing failed (error is printed to stderr).
    pub fn merge_from_json(&mut self, json: &str) -> bool {
        let result = match self {
            Commands::Scan(scan) => scan.merge_from_json(json),
            Commands::Infos(infos) => infos.merge_from_json(json),
        };
        let is_ok = result.is_ok();
        let _ = result.map_err(print_error);
        is_ok
    }

    /// Executes the command.
    ///
    /// Returns `true` on success, `false` if execution failed (error is printed to stderr).
    pub async fn execute(&self) -> bool {
        let result = match self {
            Commands::Scan(scan) => scan.execute().await,
            Commands::Infos(infos) => infos.execute().await,
        };
        let is_ok = result.is_ok();
        let _ = result.map_err(print_error);
        is_ok
    }
}

/// Defines the interface for a CLI subcommand.
pub trait Command {
    /// Merges fields from a JSON string into `self`, overriding only fields present in the JSON.
    fn merge_from_json(&mut self, json: &str) -> Result<(), Error>;

    /// Executes the command and outputs results as JSON lines to stdout.
    async fn execute(&self) -> Result<(), Error>;
}

/// Serializes `object` to a JSON string and prints it to stdout.
pub fn print_json<O: Serialize>(object: O) -> Result<(), Error> {
    println!(
        "{}",
        serde_json::to_string(&object).map_err(|_| Error::PrintOutput)?
    );
    Ok(())
}

/// Formats `error` as a Clap error and prints it to stderr.
pub fn print_error(error: Error) {
    let mut cmd = Cli::command();
    let _ = cmd.error(ErrorKind::InvalidValue, error).print();
}
