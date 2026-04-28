use crate::{
    cli::{Command, print_json},
    errors::Error,
};
use clap::Parser;
use dtiw385::{Decoder, response::ApiInfosData};
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

/// Arguments for the `infos` subcommand.
#[derive(Parser, Deserialize, Serialize, Debug, Clone)]
pub struct Infos {
    /// IPv4 address of the decoder. Can be provided via stdin JSON.
    #[arg(long)]
    ip: Option<Ipv4Addr>,
    /// Port of the decoder. Can be provided via stdin JSON.
    #[arg(long)]
    port: Option<u16>,
}

/// Flattened response combining decoder identity and its info data.
#[derive(Serialize)]
struct InfosResponse {
    #[serde(flatten)]
    decoder: Decoder,
    #[serde(flatten)]
    infos: ApiInfosData,
}

impl Command for Infos {
    /// Merges `ip` and `port` from a JSON string, giving priority to CLI arguments.
    fn merge_from_json(&mut self, json: &str) -> Result<(), Error> {
        let json = serde_json::from_str::<Self>(json)?;
        self.ip = self.ip.or(json.ip);
        self.port = self.port.or(json.port);
        Ok(())
    }

    /// Connects to the decoder and prints its info as a JSON line to stdout.
    ///
    /// # Errors
    ///
    /// Returns [`Error::MissingIp`] or [`Error::MissingPort`] if either field
    /// was not provided via CLI or stdin.
    async fn execute(&self) -> Result<(), Error> {
        let ip = self.ip.ok_or(Error::MissingIp)?;
        let port = self.port.ok_or(Error::MissingPort)?;
        let decoder = dtiw385::Decoders::connect(ip, port);
        let infos = decoder.infos().await?;
        print_json(InfosResponse { decoder, infos })?;
        Ok(())
    }
}
