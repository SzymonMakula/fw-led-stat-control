use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::env::{CONFIG_PATH_OVERRIDE_ENV, HOME_ENV, XDG_CONFIG_HOME_ENV};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Config {
    pub(crate) plugins: Vec<PluginConf>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct PluginConf {
    pub(crate) name: String,
    pub(crate) pos_x: Option<usize>,
    pub(crate) pos_y: Option<usize>,
}

impl Config {
    pub fn init() -> Self {
        let config_file = get_config_file();
        toml::from_str(&config_file).unwrap()
    }
}

/**
Use XDG config or fallback to HOME https://specifications.freedesktop.org/basedir-spec/latest/#variables
*/
fn get_config_file() -> String {
    let env_config = std::env::var(CONFIG_PATH_OVERRIDE_ENV)
        .map(PathBuf::from)
        .ok()
        .map(fs::read_to_string)
        .and_then(Result::ok);

    let default_config_dir = std::env::var(HOME_ENV)
        .map(PathBuf::from)
        .map(|mut buf| {
            buf.push(".config/fw-systemstats/config.toml");
            buf
        })
        .ok()
        .map(fs::read_to_string)
        .and_then(Result::ok);

    let xdg_config = std::env::var(XDG_CONFIG_HOME_ENV)
        .map(PathBuf::from)
        .map(|mut buf| {
            buf.push("fw-systemstats/config.toml");
            buf
        })
        .ok()
        .map(fs::read_to_string)
        .and_then(Result::ok);

    env_config
        .or(xdg_config)
        .or(default_config_dir)
        .expect(NO_CONFIG_ERR_MSG)
}

const NO_CONFIG_ERR_MSG: &str =
    "Could not resolve a configuration file. Make sure a valid configuration file \
is located at $HOME/fw-systemstats/config.toml";
