use std::env::current_exe;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use log::error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Config {
    pub(crate) plugins: Vec<PluginConf>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct PluginConf {
    pub(crate) name: String,
    pub(crate) pos_x: usize,
    pub(crate) pos_y: usize,
}

impl Config {
    pub fn init() -> Self {
        let config_file = get_config_file();
        toml::from_str(&config_file).unwrap()
    }
}

fn get_config_file() -> String {
    let config_path = current_exe()
        .map(|path| path.as_path().parent().map(PathBuf::from))
        .ok()
        .flatten()
        .map(|path| path.join("./config.toml"))
        .ok_or(std::io::Error::from(ErrorKind::Other))
        .and_then(fs::canonicalize)
        .and_then(fs::read_to_string);

    config_path.unwrap_or_else(|err| {
        match err.kind() {
            ErrorKind::NotFound => {
                error!("Configuration file not found in the library directory");
            }
            _ => {
                error!("Unknown error occurred {}", err);
            }
        }
        std::process::exit(1)
    })
}
