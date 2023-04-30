use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Config {
    pub database: Database,
}

impl Config {
    pub fn load() -> Self {
        let config_path = PathBuf::from("config.yml");
        if config_path.exists() {
            let config_content = std::fs::read_to_string(config_path).unwrap();
            serde_yaml::from_str(&config_content).unwrap()
        } else {
            panic!("Config file doesn't exist");
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Database {
    pub host: String,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl Database {
    pub fn connection_string(&self) -> String {
        let Self {
            host,
            username,
            password,
            database,
        } = self;
        format!("mysql://{}:{}@{}/{}", username, password, host, database)
    }
}
