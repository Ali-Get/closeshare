use crate::core::state::AppState;
use crate::protocol::codec;
use crate::protocol::message::{
    ChunkData, CompressionType, ProtocolMessage, TransferRequest, TransferResponse,
};
use crate::transfer::{chunker, compression, integrity};
use std::sync::Arc;
use uuid::Uuid;

pub async fn send_file(
    state: Arc<AppState>,
    transfer_id: Uuid,
    peer_addr: std::net::SocketAddr,
    file_path: String,
    group_code: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let metadata = tokio::fs::metadata(&file_path).await?;
    let file_size = metadata.len();

    let mut stream = tokio::net::TcpStream::connect(peer_addr).await?;

    let request = ProtocolMessage::TransferRequest(TransferRequest {
        transfer_id,
        device_id: state.device_id,
        group_code: group_code.clone(),
        files: vec![file_path.clone()],
        total_size: file_size,
        compression: CompressionType::Zstd { level: 1 },
        encryption: false,
    });

    codec::send_message(&mut stream, &request).await?;
    let response = codec::receive_message(&mut stream).await?;

    match response {
        ProtocolMessage::TransferResponse(TransferResponse::Accepted { .. }) => {
            let ch = chunker::Chunker::new(file_size, 1024 * 1024);
            let total_chunks = ch.total_chunks;

            for chunk_idx in 0..total_chunks {
                let chunk = ch.get_chunk(chunk_idx).unwrap();
                let data = chunker::read_chunk(&file_path, chunk.offset, chunk.size).await?;
                let compressed = compression::compress(&data)?;
                let checksum = integrity::calculate_checksum(&compressed);

                let chunk_msg = ProtocolMessage::ChunkData(ChunkData {
                    transfer_id,
                    file_index: 0,
                    chunk_index: chunk.index,
                    total_chunks: chunk.total_chunks,
                    data: compressed,
                    checksum,
                });

                codec::send_message(&mut stream, &chunk_msg).await?;
                let _ack = codec::receive_message(&mut stream).await?;
            }

            Ok(())
        }
        ProtocolMessage::TransferResponse(TransferResponse::Rejected { reason }) => {
            Err(format!("Transfer rejected: {}", reason).into())
        }
        _ => Err("Unexpected response".into()),
    }
}