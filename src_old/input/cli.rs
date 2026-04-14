use clap::{Parser, Subcommand};

pub fn parse() -> Cli {
    Cli::parse()
}

#[derive(Parser)]
#[command(name = "dtiw385")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Infos {
        #[arg(long)]
        ip: Option<String>,
        #[arg(long)]
        port: Option<String>,
    },
}
