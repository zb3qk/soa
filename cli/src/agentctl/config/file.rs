use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub output_format: Option<String>,
}

pub fn config_path() -> PathBuf {
    env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".agentctl")
        .join("config.toml")
}

impl Config {
    pub fn load() -> Self {
        let path = config_path();
        if let Ok(contents) = fs::read_to_string(&path) {
            toml::from_str(&contents).unwrap_or_default()
        } else {
            Config::default()
        }
    }

    pub fn save(&self) {
        let path = config_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let data = toml::to_string_pretty(self).expect("serialize config");
        let _ = fs::write(path, data);
    }
}
