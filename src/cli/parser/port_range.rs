use crate::errors::Error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// An inclusive range of port numbers.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct PortRange {
    /// First port in the range.
    pub start: u16,
    /// Last port in the range.
    pub end: u16,
}

impl FromStr for PortRange {
    type Err = Error;

    /// Parses a port range from a `<start>-<end>` string.
    ///
    /// # Errors
    ///
    /// Returns [`Error::PortRangeFormat`] if the separator is missing or `start > end`.
    /// Returns [`Error::InvalidStartPort`] or [`Error::InvalidEndPort`] if either value is not a valid `u16`.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (start, end) = value.split_once("-").ok_or(Error::PortRangeFormat)?;
        let (start, end) = (
            start.parse().map_err(|_| Error::InvalidStartPort)?,
            end.parse().map_err(|_| Error::InvalidEndPort)?,
        );
        if start > end {
            return Err(Error::PortRangeFormat);
        }
        Ok(Self { start, end })
    }
}
