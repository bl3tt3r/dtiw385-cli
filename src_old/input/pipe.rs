use serde::Deserialize;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::input::{CliError, cli::Command};

pub async fn parse(command: Command) -> Result<Option<Command>, CliError> {
    let stdin = tokio::io::stdin();
    let line = BufReader::new(stdin).lines().next_line().await?;
    Ok(line.map(|line| match command {
        Command::Infos { ip, port } => {
            let json: Infos = serde_json::from_str(&line).unwrap();
            Command::Infos {
                ip: ip.or(json.ip),
                port: port.or(json.port),
            }
        }
    }))
}

#[derive(Deserialize)]
pub struct Infos {
    ip: Option<String>,
    port: Option<String>,
}
