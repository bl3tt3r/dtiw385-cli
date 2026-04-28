use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::{
    cli::{Command, Ipv4Range, PortRange, print_json},
    errors::Error,
};

/// Arguments for the `scan` subcommand.
#[derive(Parser, Deserialize, Serialize, Debug, Clone)]
pub struct Scan {
    /// IPv4 address range to scan. Can be provided via stdin JSON.
    #[arg(long)]
    ip_range: Option<Ipv4Range>,
    /// Port range to scan. Can be provided via stdin JSON.
    #[arg(long)]
    port_range: Option<PortRange>,
}

impl Command for Scan {
    /// Merges `ip_range` and `port_range` from a JSON string, giving priority to CLI arguments.
    fn merge_from_json(&mut self, json: &str) -> Result<(), Error> {
        let json = serde_json::from_str::<Self>(json)?;
        self.ip_range = self.ip_range.or(json.ip_range);
        self.port_range = self.port_range.or(json.port_range);
        Ok(())
    }

    /// Scans the given IP and port ranges and prints each discovered decoder as a JSON line to stdout.
    ///
    /// # Errors
    ///
    /// Returns [`Error::MissingIpRange`] or [`Error::MissingPortRange`] if either field
    /// was not provided via CLI or stdin.
    async fn execute(&self) -> Result<(), Error> {
        let ip_range = self.ip_range.ok_or(Error::MissingIpRange)?;
        let ip_range = ip_range.start..=ip_range.end;
        let port_range = self.port_range.ok_or(Error::MissingPortRange)?;
        let port_range = port_range.start..=port_range.end;
        let mut decoders = dtiw385::Decoders::search(ip_range, port_range).find();
        while let Some(decoder) = decoders.recv().await {
            print_json(decoder)?;
        }
        Ok(())
    }
}
