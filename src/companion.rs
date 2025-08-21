// This module handles everything related to companions in terms of loading and passing over to
// game thread.

use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub companion: Vec<Companion>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Companion {
    pub name: String,
    pub path: String,
    pub width: f32,
    pub height: f32
}

pub fn load_config() -> Result<AppConfig, ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::with_name("./config/config.toml"))
        .build()
        .unwrap();

    settings.try_deserialize::<AppConfig>()
}
