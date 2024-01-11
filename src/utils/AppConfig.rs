use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub connection: String,
    pub sql_type: i32, //0:sqlite
    pub port: i32,
    pub domain: String,
}

impl AppConfig {
    pub fn get_config(file_path: &str) -> Result<AppConfig, serde_json::Error> {
        let mut file = File::open(file_path);
        if let Ok(ref mut file) = file {
            let mut contents = String::new();
            file.read_to_string(&mut contents);
            return serde_json::from_str(&contents);
        }
        let cfg = AppConfig {
            connection: "".to_string(),
            sql_type: 0,
            port: 12002,
            domain: "127.0.0.1:12002".to_string(),
        };
        Self::set_config(file_path, &cfg);
        Ok(cfg)
    }
    fn set_config(file_path: &str, config: &AppConfig) -> Result<(), serde_json::Error> {
        let mut file = File::create(file_path).unwrap();
        let json = serde_json::to_string_pretty(config)?;
        file.write_all(json.as_bytes()).unwrap();
        Ok(())
    }
}
