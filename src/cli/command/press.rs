use crate::{
    cli::{Command, Key, print_json},
    errors::Error,
};
use clap::Parser;
use dtiw385::Decoder;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

/// Arguments for the `press` subcommand.
#[derive(Parser, Deserialize, Serialize, Debug, Clone)]
pub struct Press {
    /// IPv4 address of the decoder. Can be provided via stdin JSON.
    #[arg(long)]
    ip: Option<Ipv4Addr>,
    /// Port of the decoder. Can be provided via stdin JSON.
    #[arg(long)]
    port: Option<u16>,
    /// Key to press on the decoder. Can be provided via stdin JSON.
    #[arg(long)]
    key: Option<Key>,
}

/// Outcome of a press command sent to a decoder.
#[derive(Serialize)]
enum Status {
    /// The command was accepted by the decoder.
    Ok,
    /// The command failed or the decoder did not respond.
    Ko,
}

/// Response returned after a press command, combining decoder identity and its status.
#[derive(Serialize)]
struct PressResponse {
    #[serde(flatten)]
    decoder: Decoder,
    status: Status,
}

impl Command for Press {
    /// Merges `ip`, `port`, and `key` from a JSON string, giving priority to CLI arguments.
    fn merge_from_json(&mut self, json: &str) -> Result<(), Error> {
        let json = serde_json::from_str::<Self>(json)?;
        self.ip = self.ip.or(json.ip);
        self.port = self.port.or(json.port);
        self.key = self.key.or(json.key);
        Ok(())
    }

    /// Sends a key press to the decoder and prints the result as a JSON line to stdout.
    ///
    /// # Errors
    ///
    /// Returns [`Error::MissingIp`], [`Error::MissingPort`], or [`Error::MissingKey`]
    /// if any required field was not provided via CLI or stdin.
    async fn execute(&self) -> Result<(), Error> {
        let ip = self.ip.ok_or(Error::MissingIp)?;
        let port = self.port.ok_or(Error::MissingPort)?;
        let key = self.key.ok_or(Error::MissingKey)?;
        let decoder = dtiw385::Decoders::connect(ip, port);
        let status = match decoder.press(key.code).await {
            Ok(_) => Status::Ok,
            Err(_) => Status::Ko,
        };
        print_json(PressResponse { decoder, status })?;
        Ok(())
    }
}
