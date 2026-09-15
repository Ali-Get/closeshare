use crate::core::config::Config;
use crate::discovery::peer::Peer;
use dashmap::DashMap;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct AppState {
    pub device_id: Uuid,
    pub display_name: RwLock<String>,
    pub hostname: String,
    pub config: RwLock<Config>,
    pub peers: DashMap<Uuid, Peer>,
    pub transfer_progress: DashMap<Uuid, TransferProgress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferProgress {
    pub transfer_id: Uuid,
    pub file_name: String,
    pub total_size: u64,
    pub transferred: u64,
    pub speed_mbps: f64,
    pub status: TransferStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransferStatus {
    Pending,
    Transferring,
    Completed,
    Failed(String),
    Cancelled,
}

impl AppState {
    pub fn new() -> Self {
        let config = Config::load();
        let device_id = Self::load_or_create_device_id();
        let hostname = whoami::fallible::hostname().unwrap_or_else(|_| "unknown".to_string());

        Self {
            device_id,
            display_name: RwLock::new(config.display_name.clone()),
            hostname,
            config: RwLock::new(config),
            peers: DashMap::new(),
            transfer_progress: DashMap::new(),
        }
    }

    fn load_or_create_device_id() -> Uuid {
        let id_path = dirs_next::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("closeshare")
            .join("device_id");

        if id_path.exists() {
            std::fs::read_to_string(&id_path)
                .ok()
                .and_then(|s| Uuid::parse_str(&s).ok())
                .unwrap_or_else(|| {
                    let id = Uuid::new_v4();
                    let _ = std::fs::write(&id_path, id.to_string());
                    id
                })
        } else {
            let id = Uuid::new_v4();
            if let Some(parent) = id_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = std::fs::write(&id_path, id.to_string());
            id
        }
    }
}