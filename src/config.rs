use serde::Deserialize;
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub input: InputConfig,
    pub output: OutputConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InputConfig {
    pub source: String,
    pub format: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OutputConfig {
    pub websocket: bool,
    pub rest_api: bool,
    pub seabind_log: Option<PathBuf>,
}

pub fn load_config() -> Result<Config> {
    let config_path = std::env::args()
        .nth(2)
        .unwrap_or_else(|| String::from("nexmarine.toml"));

    let config_str = std::fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_str)?;
    
    Ok(config)
} 