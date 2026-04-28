use crate::errors::Error;
use serde::{Deserialize, Serialize};
use std::{net::Ipv4Addr, str::FromStr};

/// An inclusive range of IPv4 addresses.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Ipv4Range {
    /// First address in the range.
    pub start: Ipv4Addr,
    /// Last address in the range.
    pub end: Ipv4Addr,
}

impl FromStr for Ipv4Range {
    type Err = Error;

    /// Parses an IPv4 range from a `<start>-<end>` string.
    ///
    /// # Errors
    ///
    /// Returns [`Error::IpRangeFormat`] if the separator is missing or `start > end`.
    /// Returns [`Error::InvalidStartIp`] or [`Error::InvalidEndIp`] if either address is invalid.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (start, end) = value.split_once("-").ok_or(Error::IpRangeFormat)?;
        let (start, end) = (
            start.parse().map_err(|_| Error::InvalidStartIp)?,
            end.parse().map_err(|_| Error::InvalidEndIp)?,
        );
        if start > end {
            return Err(Error::IpRangeFormat);
        }
        Ok(Self { start, end })
    }
}
