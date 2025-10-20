use std::env::current_exe;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

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
    let config_path = current_exe()
        .map(|path| path.as_path().parent().map(PathBuf::from))
        .ok()
        .flatten()
        .map(|path| path.join("config.toml"))
        .map(fs::canonicalize)
        .and_then(Result::ok)
        .unwrap();

    fs::read_to_string(config_path).unwrap()
}
