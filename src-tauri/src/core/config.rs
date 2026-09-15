use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub display_name: String,
    pub group_code: String,
    pub shared_folder: String,
    pub discovery_port: u16,
    pub transfer_port: u16,
    pub discovery_broadcast_addr: String,
    pub max_parallel_connections: u8,
    pub chunk_size_mb: u32,
    pub auto_discover: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display_name: whoami::fallible::hostname().unwrap_or_else(|_| "My Device".to_string()),
            group_code: String::new(),
            shared_folder: dirs_next::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .to_string_lossy()
                .to_string(),
            discovery_port: 19999,
            transfer_port: 20000,
            discovery_broadcast_addr: "255.255.255.255".to_string(),
            max_parallel_connections: 8,
            chunk_size_mb: 1,
            auto_discover: true,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = get_config_path();
        if config_path.exists() {
            std::fs::read_to_string(&config_path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            let config = Self::default();
            config.save().ok();
            config
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = get_config_path();
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(config_path, json)?;
        Ok(())
    }
}

fn get_config_path() -> PathBuf {
    dirs_next::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("closeshare")
        .join("config.json")
}