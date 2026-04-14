use std::io::{self, IsTerminal};

use crate::{CliError, Infos};

pub mod pipe;
pub mod raw;

pub fn error(error: CliError) {
    if io::stdout().is_terminal() {
        raw::error(error);
    } else {
        pipe::error(error);
    }
}

pub fn infos(infos: Infos) {
    if io::stdout().is_terminal() {
        raw::infos(infos);
    } else {
        pipe::infos(infos);
    }
}
