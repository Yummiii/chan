use figment::{
    Figment,
    providers::{Format, Toml},
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub sharo: Sharo,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Sharo {
    pub jwt_secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

impl Config {
    pub fn get() -> Self {
        Figment::new()
            .merge(Toml::file("config.toml"))
            .extract()
            .expect("Failed to load config")
    }
}
