use crate::errors::Error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Key {
    pub code: u16,
}

impl FromStr for Key {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let key = match value {
            "PowerOnOff" => Ok(dtiw385::key::Key::PowerOnOff),
            "Ok" => Ok(dtiw385::key::Key::Ok),
            "Up" => Ok(dtiw385::key::Key::Up),
            "Down" => Ok(dtiw385::key::Key::Down),
            "Left" => Ok(dtiw385::key::Key::Left),
            "Right" => Ok(dtiw385::key::Key::Right),
            "Back" => Ok(dtiw385::key::Key::Back),
            "Menu" => Ok(dtiw385::key::Key::Menu),
            "VolumeUp" => Ok(dtiw385::key::Key::VolumeUp),
            "VolumeDown" => Ok(dtiw385::key::Key::VolumeDown),
            "Mute" => Ok(dtiw385::key::Key::Mute),
            "ChannelUp" => Ok(dtiw385::key::Key::ChannelUp),
            "ChannelDown" => Ok(dtiw385::key::Key::ChannelDown),
            "Play" => Ok(dtiw385::key::Key::Play),
            "Pause" => Ok(dtiw385::key::Key::Pause),
            "Stop" => Ok(dtiw385::key::Key::Stop),
            "Forward" => Ok(dtiw385::key::Key::Forward),
            "Rewind" => Ok(dtiw385::key::Key::Rewind),
            "N0" => Ok(dtiw385::key::Key::N0),
            "N1" => Ok(dtiw385::key::Key::N1),
            "N2" => Ok(dtiw385::key::Key::N2),
            "N3" => Ok(dtiw385::key::Key::N3),
            "N4" => Ok(dtiw385::key::Key::N4),
            "N5" => Ok(dtiw385::key::Key::N5),
            "N6" => Ok(dtiw385::key::Key::N6),
            "N7" => Ok(dtiw385::key::Key::N7),
            "N8" => Ok(dtiw385::key::Key::N8),
            "N9" => Ok(dtiw385::key::Key::N9),
            _ => Err(Error::InvalidKey),
        }?;
        Ok(Key { code: key.into() })
    }
}
