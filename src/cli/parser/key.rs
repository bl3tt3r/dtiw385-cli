use crate::errors::Error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Key {
    pub code: Key,
}

impl FromStr for Key {
    type Err = Error;

    /// Parses an IPv4 range from a `<start>-<end>` string.
    ///
    /// # Errors
    ///
    /// Returns [`Error::IpRangeFormat`] if the separator is missing or `start > end`.
    /// Returns [`Error::InvalidStartIp`] or [`Error::InvalidEndIp`] if either address is invalid.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let Key = match value {
            "PowerOnOff" => Ok(dtiw385::key::Key::TOTO),
            "Ok" => Ok(dtiw385::key::Key::TOTO),
            "Up" => Ok(dtiw385::key::Key::TOTO),
            "Down" => Ok(dtiw385::key::Key::TOTO),
            "Left" => Ok(dtiw385::key::Key::TOTO),
            "Right" => Ok(dtiw385::key::Key::TOTO),
            "Back" => Ok(dtiw385::key::Key::TOTO),
            "Menu" => Ok(dtiw385::key::Key::TOTO),
            "VolumeUp" => Ok(dtiw385::key::Key::TOTO),
            "VolumeDown" => Ok(dtiw385::key::Key::TOTO),
            "Mute" => Ok(dtiw385::key::Key::TOTO),
            "ChannelUp" => Ok(dtiw385::key::Key::TOTO),
            "ChannelDown" => Ok(dtiw385::key::Key::TOTO),
            "Play" => Ok(dtiw385::key::Key::TOTO),
            "Pause" => Ok(dtiw385::key::Key::TOTO),
            "Stop" => Ok(dtiw385::key::Key::TOTO),
            "Forward" => Ok(dtiw385::key::Key::TOTO),
            "Rewind" => Ok(dtiw385::key::Key::TOTO),
            "N0" => Ok(dtiw385::key::Key::TOTO),
            "N1" => Ok(dtiw385::key::Key::TOTO),
            "N2" => Ok(dtiw385::key::Key::TOTO),
            "N3" => Ok(dtiw385::key::Key::TOTO),
            "N4" => Ok(dtiw385::key::Key::TOTO),
            "N5" => Ok(dtiw385::key::Key::TOTO),
            "N6" => Ok(dtiw385::key::Key::TOTO),
            "N7" => Ok(dtiw385::key::Key::TOTO),
            "N8" => Ok(dtiw385::key::Key::TOTO),
            "N9" => Ok(dtiw385::key::Key::TOTO),
            _ => Err(Error::InvalidKey),
        }?;
    }
}
