use crate::CliError;
use std::{
    io::{self, IsTerminal},
    net::Ipv4Addr,
    sync::{Mutex, OnceLock},
};
pub mod cli;
pub mod pipe;

static TERMINAL_ALREADY_GET: Mutex<bool> = Mutex::new(false);

pub enum Command {
    Infos { ip: Ipv4Addr, port: u16 },
}

pub async fn commandes() -> Result<Option<Command>, CliError> {
    let cli = cli::parse();
    let mut cli_command = cli.command;

    if !io::stdin().is_terminal() {
        match pipe::parse(cli_command).await? {
            Some(command) => cli_command = command,
            None => return Ok(None),
        }
    } else {
        let mut already = TERMINAL_ALREADY_GET.lock().unwrap();
        if *already {
            return Ok(None);
        }
        *already = true;
    }

    let command = match cli_command {
        cli::Command::Infos { ip, port } => Command::Infos {
            ip: ip.ok_or(CliError::EmptyIp)?.parse()?,
            port: port.ok_or(CliError::EmptyPort)?.parse()?,
        },
    };

    Ok(Some(command))
}
