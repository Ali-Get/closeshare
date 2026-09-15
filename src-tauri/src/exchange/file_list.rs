use crate::core::state::AppState;
use crate::exchange::scanner;
use crate::protocol::codec;
use crate::protocol::message::{FileListResponse, FileMetadata, ProtocolMessage};
use std::sync::Arc;

pub async fn request_file_list(
    peer_addr: std::net::SocketAddr,
    group_code: &str,
    device_id: uuid::Uuid,
    path: Option<String>,
) -> Result<FileListResponse, Box<dyn std::error::Error + Send + Sync>> {
    use crate::protocol::message::FileListRequest;

    let mut stream = tokio::net::TcpStream::connect(peer_addr).await?;

    let request = ProtocolMessage::FileListRequest(FileListRequest {
        device_id,
        group_code: group_code.to_string(),
        path,
    });

    codec::send_message(&mut stream, &request).await?;
    let response = codec::receive_message(&mut stream).await?;

    match response {
        ProtocolMessage::FileListResponse(resp) => Ok(resp),
        _ => Err("Unexpected response type".into()),
    }
}

pub async fn handle_file_list_request(
    state: Arc<AppState>,
    mut stream: tokio::net::TcpStream,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let message = codec::receive_message(&mut stream).await?;

    match message {
        ProtocolMessage::FileListRequest(request) => {
            let config = state.config.read();

            if request.group_code != config.group_code {
                let _ = codec::send_message(
                    &mut stream,
                    &ProtocolMessage::FileListResponse(FileListResponse {
                        device_id: state.device_id,
                        display_name: state.display_name.read().clone(),
                        files: vec![],
                        path: String::new(),
                    }),
                ).await;
                return Ok(());
            }

            let scan_path = if let Some(ref sub_path) = request.path {
                std::path::Path::new(&config.shared_folder).join(sub_path)
            } else {
                std::path::Path::new(&config.shared_folder).to_path_buf()
            };

            let files = scanner::scan_directory(&scan_path);

            let metadata: Vec<FileMetadata> = files
                .into_iter()
                .map(|f| FileMetadata {
                    name: f.name,
                    path: f.relative_path,
                    size: f.size,
                    is_directory: f.is_directory,
                    modified: f.modified,
                    checksum: f.checksum,
                    mime_type: f.mime_type,
                })
                .collect();

            let response = ProtocolMessage::FileListResponse(FileListResponse {
                device_id: state.device_id,
                display_name: state.display_name.read().clone(),
                files: metadata,
                path: scan_path.to_string_lossy().to_string(),
            });

            codec::send_message(&mut stream, &response).await?;
            Ok(())
        }
        _ => Err("Expected FileListRequest".into()),
    }
}