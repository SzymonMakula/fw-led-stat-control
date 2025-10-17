use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::env::{CONFIG_DIR_OVERRIDE_ENV, HOME_ENV, XDG_CONFIG_HOME_ENV};

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
        let config_path = get_config_path();
        let toml_str = fs::read_to_string(config_path).expect(NO_CONFIG_ERR_MSG);
        toml::from_str(&toml_str).unwrap()
    }
}

/**
Use XDG config or fallback to HOME https://specifications.freedesktop.org/basedir-spec/latest/#variables
*/
fn get_config_path() -> PathBuf {
    let env_config_dir = std::env::var(CONFIG_DIR_OVERRIDE_ENV).map(PathBuf::from);

    let mut default_config_dir_path =
        PathBuf::from(std::env::var(HOME_ENV).expect("Missing 'HOME' env variable"));
    default_config_dir_path.push(".config/fw-systemstats/config.toml");

    let mut xdg_config_path =
        std::env::var(XDG_CONFIG_HOME_ENV)
            .map(PathBuf::from)
            .map(|mut buf| {
                buf.push("fw-systemstats/config.toml");
                buf
            });

    env_config_dir
        .or(xdg_config_path)
        .unwrap_or(default_config_dir_path)
}

const NO_CONFIG_ERR_MSG: &str =
    "Could not resolve a configuration file. Make sure a valid configuration file \
is located at $HOME/fw-systemstats/config.toml";
