use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    repo: String,
    anvl_storage_path: String,
    always_push: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            repo: String::new(),
            anvl_storage_path: String::new(),
            always_push: false,
        }
    }
}

impl Config {
    fn path() -> Result<PathBuf, String> {
        let mut path = dirs::config_dir().ok_or("could not determine config directory")?;

        path.push("anvl");
        path.push("config.json");
        Ok(path)
    }
}

impl Config {
    pub fn get() -> Result<Self, String> {
        let path = Self::path()?;
        let json =
            fs::read_to_string(&path).map_err(|e| format!("failed to read config file {e}"))?;

        serde_json::from_str(&json)
            .map_err(|e| format!("failed to parse config file {:?}: {}", path, e))
    }
}

impl Config {
    pub fn set(key: String, value: String) -> Result<(), String> {
        let mut config = Self::get()?;

        match key.as_str() {
            "repo" => config.repo = value.to_string(),
            "anvl_storage_path" => config.anvl_storage_path = value.to_string(),
            "always_push" => {
                config.always_push = value
                    .parse::<bool>()
                    .map_err(|_| "always_push must be true or false".to_string())?;
            }
            _ => return Err(format!("unknow config key: {key}")),
        }
        Ok(())
    }
}

impl Config {
    pub fn save(&self) -> Result<(), String> {
        let path = Self::path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("failed to create config directory: {e}"))?;
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("failed to serialize config: {e}"))?;

        fs::write(&path, json).map_err(|e| format!("failed to write config file: {e}"))?;
        Ok(())
    }
}
