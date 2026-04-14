use std::{net::AddrParseError, num::ParseIntError};

use dtiw385::DecoderError;
use serde::Serialize;
use thiserror::Error;

mod input;
mod output;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Impossible de parser l'adresse IP.")]
    IpParse(#[from] AddrParseError),
    #[error("Impossible de parser le port.")]
    PortParse(#[from] ParseIntError),
    #[error("Erreur de decoder : {0}")]
    Decoder(#[from] DecoderError),
    #[error("Impossible de lire l'entrée pipé.")]
    Io(#[from] std::io::Error),
    #[error("L'adresse Ip ne peut pas etre vide.")]
    EmptyIp,
    #[error("Le port ne peut pas etre vide.")]
    EmptyPort,
}

#[tokio::main]
async fn main() {
    loop {
        match input::commandes().await {
            // Une nouvelle commande est recu
            Ok(Some(command)) => perform_command(command).await,
            // La commande en cours a rencontré une erreur
            Err(error) => {
                output::error(error);
                break;
            }
            // Plus de commande a traiter
            Ok(None) => break,
        }
    }
}

#[derive(Serialize)]
pub struct Infos {
    pub played_media_type: String,
    pub played_media_state: String,
    pub played_media_id: String,
    pub played_media_context_id: String,
    pub played_media_position: String,
    pub time_shifting_state: String,
    pub mac_address: String,
    pub wol_support: String,
    pub friendly_name: String,
    pub active_standby_state: String,
    pub npvr_support: String,
}

async fn perform_command(command: input::Command) {
    match command {
        input::Command::Infos { ip, port } => {
            match dtiw385::Decoders::connect(ip, port).infos().await {
                Ok(infos) => {
                    output::infos(Infos {
                        played_media_type: infos.played_media_type,
                        played_media_state: infos.played_media_state,
                        played_media_id: infos.played_media_id,
                        played_media_context_id: infos.played_media_context_id,
                        played_media_position: infos.played_media_position,
                        time_shifting_state: infos.time_shifting_state,
                        mac_address: infos.mac_address,
                        wol_support: infos.wol_support,
                        friendly_name: infos.friendly_name,
                        active_standby_state: infos.active_standby_state,
                        npvr_support: infos.npvr_support,
                    });
                }
                Err(error) => output::error(error.into()),
            }
        }
    }
}
