use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub device_id: Uuid,
    pub display_name: String,
    pub hostname: String,
    pub socket_addr: SocketAddr,
    pub transfer_port: u16,
    pub last_seen: DateTime<Utc>,
    pub status: PeerStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PeerStatus {
    Online,
    Away,
    Offline,
}

impl Peer {
    pub fn new(
        device_id: Uuid,
        display_name: String,
        hostname: String,
        socket_addr: SocketAddr,
        transfer_port: u16,
    ) -> Self {
        Self {
            device_id,
            display_name,
            hostname,
            socket_addr,
            transfer_port,
            last_seen: Utc::now(),
            status: PeerStatus::Online,
        }
    }
}