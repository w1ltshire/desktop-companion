//! This module handles loading companion-related configuration and passing it to the game thread.
//!
//! It provides structures representing companions, their sprites, and animation configurations,
//! as well as functions to load configuration from TOML files.

use std::collections::HashMap;
use config::{Config, ConfigError};
use serde::Deserialize;

/// Root application configuration containing all companions.
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    /// List of companions configured in the app.
    pub companion: Vec<Companion>,
}

/// Basic companion metadata.
#[derive(Debug, Deserialize, Clone)]
pub struct Companion {
    /// Name of the companion.
    pub name: String,
    /// Path to the companion's configuration or assets.
    pub path: String,
    /// Width of the companion sprite in pixls.
    pub width: f32,
    /// Height of the companion sprite in pixels.
    pub height: f32,
    /// Walking speed of the companion.
    pub walkspeed: f32,
}

/// Single sprite for an animation.
#[derive(Debug, Deserialize, Clone)]
pub struct Sprite {
    /// Path to the image file for the sprite.
    pub path: String,
}

/// Configuration for a single companion's animations.
#[derive(Debug, Deserialize, Clone)]
pub struct CompanionConfig {
    /// Map of animation names to lists of sprites.
    pub animations: HashMap<String, Vec<Sprite>>,
}

/// Loads the main application configuration from `./config/config.toml`.
///
/// # Returns
/// * `Ok(AppConfig)` if the file exists and deserializes successfully.
/// * `Err(ConfigError)` if the file cannot be read or deserialized.
pub fn load_config() -> Result<AppConfig, ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::with_name("./config/config.toml"))
        .build()
        .unwrap();

    settings.try_deserialize::<AppConfig>()
}

/// Loads a companion's animation configuration from a specified file path.
///
/// # Arguments
/// * `path` - Path to the TOML file containing companion animations.
///
/// # Returns
/// * `Ok(CompanionConfig)` if the file exists and deserializes successfully.
/// * `Err(ConfigError)` if the file cannot be read or deserialized.
pub fn load_companion_config(path: &str) -> Result<CompanionConfig, ConfigError> {
    let settings = Config::builder()
        .add_source(config::File::with_name(path))
        .build()
        .unwrap();

    settings.try_deserialize::<CompanionConfig>()
}

