use clap::Args;
use serde::Deserialize;

use crate::commands::Merge;

#[derive(Args, Deserialize, Clone)]
pub struct Infos {
    #[arg(long)]
    ip: Option<String>,
    #[arg(long)]
    port: Option<String>,
}

impl Merge for Infos {
    fn merge(&mut self, other: Self) {
        self.ip = other.ip.or(self.ip.take());
        self.port = other.port.or(self.port.take());
    }
}
