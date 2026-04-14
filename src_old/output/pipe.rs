use crate::{CliError, Infos};
use serde::Serialize;

#[derive(Serialize)]
pub struct Error {
    message: String,
}

pub fn error(error: CliError) {
    let error = Error {
        message: error.to_string(),
    };
    let json = serde_json::to_string(&error).unwrap();
    println!("{}", json);
}

pub fn infos(infos: Infos) {
    let json = serde_json::to_string(&infos).unwrap();
    println!("{}", json);
}
