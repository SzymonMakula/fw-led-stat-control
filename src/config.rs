use std::collections::HashMap;
use std::fs;

use serde::Deserialize;

use crate::canvas::Canvas;

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) plugins: Vec<PluginConf>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct PluginConf {
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) pos_x: Option<usize>,
    pub(crate) pos_y: Option<usize>,
}

impl Config {
    pub fn init() -> Self {
        let toml_str = fs::read_to_string("config.toml").unwrap();
        toml::from_str(&toml_str).unwrap()
    }
}
