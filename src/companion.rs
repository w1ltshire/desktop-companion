// This module handles everything related to companions in terms of loading and passing over to
// game thread.

use std::collections::HashMap;

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
    pub height: f32,
    pub walkspeed: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Sprite {
    pub path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CompanionConfig {
    pub animations: HashMap<String, Vec<Sprite>>,
}

pub fn load_config() -> Result<AppConfig, ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::with_name("./config/config.toml"))
        .build()
        .unwrap();

    settings.try_deserialize::<AppConfig>()
}

pub fn load_companion_config(path: &str) -> Result<CompanionConfig, ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::with_name(path))
        .build()
        .unwrap();

    settings.try_deserialize::<CompanionConfig>()
}
