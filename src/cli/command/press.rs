use crate::{
    cli::{Command, Key, print_json},
    errors::Error,
};
use clap::Parser;
use dtiw385::Decoder;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

#[derive(Parser, Deserialize, Serialize, Debug, Clone)]
pub struct Press {
    #[arg(long)]
    ip: Option<Ipv4Addr>,
    #[arg(long)]
    port: Option<u16>,
    #[arg(long)]
    key: Option<Key>,
}

#[derive(Serialize)]
enum Status {
    Ok,
    Ko,
}

#[derive(Serialize)]
struct PressResponse {
    #[serde(flatten)]
    decoder: Decoder,
    status: Status,
}

impl Command for Press {
    fn merge_from_json(&mut self, json: &str) -> Result<(), Error> {
        let json = serde_json::from_str::<Self>(json)?;
        self.ip = self.ip.or(json.ip);
        self.port = self.port.or(json.port);
        self.key = self.key.or(json.key);
        Ok(())
    }

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
