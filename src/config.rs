use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub location: Location,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let config_path = Self::get_config_path()?;

        let content = fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read config file at {:?}: {}", config_path, e))?;

        toml::from_str(&content).map_err(|e| format!("Failed to parse config: {}", e))
    }

    fn get_config_path() -> Result<PathBuf, String> {
        let config_dir = if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
            PathBuf::from(xdg_config)
        } else {
            dirs::config_dir().ok_or("Could not determine config directory")?
        };

        Ok(config_dir.join("weathr").join("config.toml"))
    }
}
