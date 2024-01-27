use crate::AppConfig;
use rand::Rng;
use rusqlite::{params, Connection, Result};
pub struct common;
impl common {
    pub fn get_conn() -> Result<Connection, rusqlite::Error> {
        let cfg = AppConfig::get_config("./config.json").unwrap();
        let conn = Connection::open(cfg.connection)?;
        Ok(conn)
    }
    pub fn get_random(length: usize) -> String {
        let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut rng = rand::thread_rng();

        let mut result = String::with_capacity(length);
        for _ in 0..length {
            let index = rng.gen_range(0..charset.len());
            result.push(charset.chars().nth(index).unwrap());
        }

        result
    }
    pub fn now() -> String {
        let now = chrono::Utc::now()
            .naive_utc()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        now
    }
}
