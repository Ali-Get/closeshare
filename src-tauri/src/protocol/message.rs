use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::net::SocketAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProtocolMessage {
    Discovery(DiscoveryMessage),
    DiscoveryResponse(DiscoveryResponse),
    FileListRequest(FileListRequest),
    FileListResponse(FileListResponse),
    TransferRequest(TransferRequest),
    TransferResponse(TransferResponse),
    ChunkData(ChunkData),
    ChunkAck(ChunkAck),
    TransferComplete(TransferComplete),
    TransferError(TransferError),
    Ping,
    Pong,
    Goodbye(GoodbyeMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryMessage {
    pub device_id: Uuid,
    pub display_name: String,
    pub hostname: String,
    pub group_code: String,
    pub transfer_port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryResponse {
    pub device_id: Uuid,
    pub display_name: String,
    pub hostname: String,
    pub group_code: String,
    pub socket_addr: SocketAddr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileListRequest {
    pub device_id: Uuid,
    pub group_code: String,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileListResponse {
    pub device_id: Uuid,
    pub display_name: String,
    pub files: Vec<FileMetadata>,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub is_directory: bool,
    pub modified: String,
    pub checksum: Option<String>,
    pub mime_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRequest {
    pub transfer_id: Uuid,
    pub device_id: Uuid,
    pub group_code: String,
    pub files: Vec<String>,
    pub total_size: u64,
    pub compression: CompressionType,
    pub encryption: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionType {
    None,
    Zstd { level: i32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransferResponse {
    Accepted {
        transfer_id: Uuid,
        port: u16,
        max_parallel_connections: u8,
    },
    Rejected {
        reason: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkData {
    pub transfer_id: Uuid,
    pub file_index: u32,
    pub chunk_index: u64,
    pub total_chunks: u64,
    pub data: Vec<u8>,
    pub checksum: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkAck {
    pub transfer_id: Uuid,
    pub file_index: u32,
    pub chunk_index: u64,
    pub status: ChunkStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChunkStatus {
    Received,
    Corrupted,
    Duplicate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferComplete {
    pub transfer_id: Uuid,
    pub success: bool,
    pub total_bytes: u64,
    pub duration_ms: u64,
    pub average_speed_mbps: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferError {
    pub transfer_id: Uuid,
    pub error_type: TransferErrorType,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransferErrorType {
    FileNotFound,
    PermissionDenied,
    DiskFull,
    ConnectionLost,
    ChecksumMismatch,
    Cancelled,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoodbyeMessage {
    pub device_id: Uuid,
}