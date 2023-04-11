use std::{fs::read_to_string, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub redis: Redis,
}

impl Config {
    /// Load contents from local config file.
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = PathBuf::from("config.yml");
        if !config_path.exists() {
            return Err("`config.yml` doesn't exist!".into());
        }

        return if let Ok(config_contents) = read_to_string(config_path) {
            match serde_yaml::from_str(&config_contents) {
                Ok(config) => Ok(config),
                Err(e) => Err(e.into()),
            }
        } else {
            Err("Unable to read `config.yml`".into())
        };
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Redis {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: String,
    pub select: Option<usize>,
}
