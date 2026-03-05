use std::path::Path;

use serde::Deserialize;
use tracing::info;

const DEFAULT_CONFIG_PATH: &str = "/opt/cityfarm/config.toml";

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_backend_url")]
    pub backend_url: String,
    #[serde(default)]
    pub api_key: String,
    #[serde(default = "default_read_interval")]
    pub read_interval_secs: u64,
    #[serde(default = "default_calibration_path")]
    pub calibration_path: String,
    #[serde(default = "default_db_path")]
    pub db_path: String,
    #[serde(default)]
    pub relay: RelayConfig,
}

#[derive(Debug, Deserialize)]
pub struct RelayConfig {
    #[serde(default = "default_gpio_pin")]
    pub gpio_pin: u8,
    #[serde(default = "default_max_duration")]
    pub max_duration_secs: u64,
    #[serde(default = "default_cooldown")]
    pub cooldown_secs: u64,
}

impl Default for RelayConfig {
    fn default() -> Self {
        Self {
            gpio_pin: default_gpio_pin(),
            max_duration_secs: default_max_duration(),
            cooldown_secs: default_cooldown(),
        }
    }
}

fn default_backend_url() -> String {
    "http://localhost:8080".to_string()
}
fn default_read_interval() -> u64 {
    10
}
fn default_calibration_path() -> String {
    "/opt/cityfarm/calibration.json".to_string()
}
fn default_db_path() -> String {
    "/opt/cityfarm/buffer.db".to_string()
}
fn default_gpio_pin() -> u8 {
    17
}
fn default_max_duration() -> u64 {
    300
}
fn default_cooldown() -> u64 {
    60
}

impl Config {
    pub fn load(path: Option<&str>) -> anyhow::Result<Self> {
        let path = path.unwrap_or(DEFAULT_CONFIG_PATH);

        if !Path::new(path).exists() {
            anyhow::bail!("Config file not found: {}", path);
        }

        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        info!("Loaded config from {}", path);
        Ok(config)
    }
}
