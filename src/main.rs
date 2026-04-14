use crate::commands::{Infos, Merge};
use clap::{Parser, Subcommand};
use std::io::{IsTerminal, stdin, stdout};
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, BufReader};

pub mod commands;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to read input from pipe.")]
    IoIn(#[from] std::io::Error),
    #[error("Unable to parse input json.")]
    IoInJson(#[from] serde_json::Error),
}

#[tokio::main]
async fn main() {
    let instance = Instance::default();
}

pub struct Instance {
    io_in_is_pipe: bool,
    io_out_is_pipe: bool,
    command: Command,
    count: i32,
}

impl Default for Instance {
    fn default() -> Self {
        let cli = Cli::parse();
        Self {
            io_in_is_pipe: !stdin().is_terminal(),
            io_out_is_pipe: !stdout().is_terminal(),
            command: cli.command,
            count: 0,
        }
    }
}

impl Instance {
    pub async fn parse(&mut self) -> Result<Option<Command>, Error> {
        let mut command = self.command.clone();
        if self.io_in_is_pipe {
            let stdin = tokio::io::stdin();
            if let Some(line) = BufReader::new(stdin).lines().next_line().await? {
                command.merge_from_json(&line)?;
            } else {
                return Ok(None);
            }
        } else if self.count > 1 {
            return Ok(None);
        }
        self.count += 1;
        Ok(Some(command))
    }
}

#[derive(Parser)]
#[command(name = "dtiw385")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Clone)]
pub enum Command {
    Infos(Infos),
}

impl Command {
    fn merge_from_json(&mut self, line: &str) -> Result<(), Error> {
        match self {
            Command::Infos(infos) => {
                infos.merge(serde_json::from_str(line)?);
            }
        }
        Ok(())
    }
}
