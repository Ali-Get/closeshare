use crate::core::state::AppState;
use crate::protocol::codec;
use crate::protocol::message::{ChunkAck, ChunkStatus, ProtocolMessage, TransferComplete, TransferResponse};
use crate::transfer::{chunker, compression, integrity};
use std::sync::Arc;
use uuid::Uuid;
use std::time::Instant;

pub async fn receive_file(
    state: Arc<AppState>,
    mut stream: tokio::net::TcpStream,
    save_dir: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start_time = Instant::now();
    let mut total_bytes = 0u64;

    let request = codec::receive_message(&mut stream).await?;

    match request {
        ProtocolMessage::TransferRequest(req) => {
            let config = state.config.read();

            if req.group_code != config.group_code {
                let response = ProtocolMessage::TransferResponse(
                    TransferResponse::Rejected {
                        reason: "Invalid group code".to_string(),
                    },
                );
                codec::send_message(&mut stream, &response).await?;
                return Ok(());
            }

            let response = ProtocolMessage::TransferResponse(
                TransferResponse::Accepted {
                    transfer_id: req.transfer_id,
                    port: 0,
                    max_parallel_connections: 8,
                },
            );
            codec::send_message(&mut stream, &response).await?;

            let file_name = req.files.first()
                .and_then(|f| std::path::Path::new(f).file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("received_file");

            let save_path = std::path::Path::new(&save_dir).join(file_name);

            loop {
                let message = codec::receive_message(&mut stream).await?;

                match message {
                    ProtocolMessage::ChunkData(chunk) => {
                        let decompressed = match compression::decompress(&chunk.data) {
                            Ok(d) => d,
                            Err(e) => {
                                tracing::error!("Decompression error: {}", e);
                                let ack = ProtocolMessage::ChunkAck(ChunkAck {
                                    transfer_id: chunk.transfer_id,
                                    file_index: chunk.file_index,
                                    chunk_index: chunk.chunk_index,
                                    status: ChunkStatus::Corrupted,
                                });
                                codec::send_message(&mut stream, &ack).await?;
                                continue;
                            }
                        };

                        if !integrity::verify_checksum(&chunk.data, &chunk.checksum) {
                            let ack = ProtocolMessage::ChunkAck(ChunkAck {
                                transfer_id: chunk.transfer_id,
                                file_index: chunk.file_index,
                                chunk_index: chunk.chunk_index,
                                status: ChunkStatus::Corrupted,
                            });
                            codec::send_message(&mut stream, &ack).await?;
                            continue;
                        }

                        let offset = chunk.chunk_index * 1024 * 1024;
                        chunker::write_chunk(
                            save_path.to_string_lossy().as_ref(),
                            offset,
                            &decompressed,
                        ).await?;

                        total_bytes += decompressed.len() as u64;

                        let ack = ProtocolMessage::ChunkAck(ChunkAck {
                            transfer_id: chunk.transfer_id,
                            file_index: chunk.file_index,
                            chunk_index: chunk.chunk_index,
                            status: ChunkStatus::Received,
                        });
                        codec::send_message(&mut stream, &ack).await?;

                        if chunk.chunk_index >= chunk.total_chunks - 1 {
                            let duration = start_time.elapsed();
                            let speed_mbps = (total_bytes as f64 * 8.0) / (duration.as_secs_f64() * 1_000_000.0);

                            let complete = ProtocolMessage::TransferComplete(TransferComplete {
                                transfer_id: chunk.transfer_id,
                                success: true,
                                total_bytes,
                                duration_ms: duration.as_millis() as u64,
                                average_speed_mbps: speed_mbps,
                            });

                            codec::send_message(&mut stream, &complete).await?;
                            break;
                        }
                    }
                    _ => {
                        tracing::warn!("Unexpected message during transfer");
                    }
                }
            }

            Ok(())
        }
        _ => Err("Expected TransferRequest".into()),
    }
}