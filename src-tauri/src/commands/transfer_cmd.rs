use crate::core::state::{AppState, TransferProgress, TransferStatus};
use crate::transfer::sender;
use std::sync::Arc;
use uuid::Uuid;

#[tauri::command]
pub async fn send_file(
    state: tauri::State<'_, Arc<AppState>>,
    peer_device_id: String,
    file_path: String,
) -> Result<String, String> {
    let state = state.inner().clone();
    let transfer_id = Uuid::new_v4();
    let peer_id = Uuid::parse_str(&peer_device_id)
        .map_err(|e| format!("Invalid peer ID: {}", e))?;
    let peer = state.peers.get(&peer_id)
        .ok_or_else(|| "Peer not found".to_string())?;
    let peer_addr = peer.socket_addr;
    let group_code = state.config.read().group_code.clone();

    let file_name = std::path::Path::new(&file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    state.transfer_progress.insert(transfer_id, TransferProgress {
        transfer_id,
        file_name: file_name.to_string(),
        total_size: 0,
        transferred: 0,
        speed_mbps: 0.0,
        status: TransferStatus::Pending,
    });

    let state_clone = state.clone();
    let file_path_clone = file_path.clone();
    let transfer_id_clone = transfer_id;

    tokio::spawn(async move {
        match sender::send_file(
            state_clone.clone(),
            transfer_id_clone,
            peer_addr,
            file_path_clone,
            group_code,
        ).await {
            Ok(_) => {
                if let Some(mut progress) = state_clone.transfer_progress.get_mut(&transfer_id_clone) {
                    progress.status = TransferStatus::Completed;
                }
            }
            Err(e) => {
                if let Some(mut progress) = state_clone.transfer_progress.get_mut(&transfer_id_clone) {
                    progress.status = TransferStatus::Failed(e.to_string());
                }
            }
        }
    });

    Ok(transfer_id.to_string())
}

#[tauri::command]
pub async fn send_folder(
    state: tauri::State<'_, Arc<AppState>>,
    peer_device_id: String,
    folder_path: String,
) -> Result<String, String> {
    let state = state.inner().clone();
    let transfer_id = Uuid::new_v4();
    
    let peer_id = Uuid::parse_str(&peer_device_id)
        .map_err(|e| format!("Invalid peer ID: {}", e))?;
    
    let peer = state.peers.get(&peer_id)
        .ok_or_else(|| "Peer not found".to_string())?;
    
    let peer_addr = peer.socket_addr;
    let group_code = state.config.read().group_code.clone();
    
    let folder_name = std::path::Path::new(&folder_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("folder");

    let temp_dir = std::env::temp_dir();
    let zip_path = temp_dir.join(format!("{}.zip", folder_name));
    
    let folder_path_clone = folder_path.clone();
    let zip_path_clone = zip_path.clone();
    
    // ضغط المجلد
    let _zip_result = tokio::task::spawn_blocking(move || {
        let file = std::fs::File::create(&zip_path_clone).map_err(|e| e.to_string())?;
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        
        fn add_dir(
            zip: &mut zip::ZipWriter<std::fs::File>,
            path: &std::path::Path,
            prefix: &std::path::Path,
            options: zip::write::FileOptions,
        ) -> Result<(), String> {
            for entry in std::fs::read_dir(path).map_err(|e| e.to_string())? {
                let entry = entry.map_err(|e| e.to_string())?;
                let entry_path = entry.path();
                let name = prefix.join(entry.file_name());
                
                if entry_path.is_dir() {
                    zip.add_directory(
                        name.to_string_lossy().replace('\\', "/"),
                        options,
                    ).map_err(|e| e.to_string())?;
                    add_dir(zip, &entry_path, &name, options)?;
                } else {
                    zip.start_file(
                        name.to_string_lossy().replace('\\', "/"),
                        options,
                    ).map_err(|e| e.to_string())?;
                    let data = std::fs::read(&entry_path).map_err(|e| e.to_string())?;
                    use std::io::Write;
                    zip.write_all(&data).map_err(|e| e.to_string())?;
                }
            }
            Ok(())
        }
        
        add_dir(&mut zip, std::path::Path::new(&folder_path_clone), std::path::Path::new(""), options)?;
        zip.finish().map_err(|e| e.to_string())?;
        Ok::<_, String>(())
    }).await.map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;

    state.transfer_progress.insert(transfer_id, TransferProgress {
        transfer_id,
        file_name: format!("{}.zip", folder_name),
        total_size: 0,
        transferred: 0,
        speed_mbps: 0.0,
        status: TransferStatus::Pending,
    });

    let state_clone = state.clone();
    let zip_path_str = zip_path.to_string_lossy().to_string();
    let transfer_id_clone = transfer_id;

    tokio::spawn(async move {
        match sender::send_file(
            state_clone.clone(),
            transfer_id_clone,
            peer_addr,
            zip_path_str.clone(),
            group_code,
        ).await {
            Ok(_) => {
                if let Some(mut progress) = state_clone.transfer_progress.get_mut(&transfer_id_clone) {
                    progress.status = TransferStatus::Completed;
                }
            }
            Err(e) => {
                if let Some(mut progress) = state_clone.transfer_progress.get_mut(&transfer_id_clone) {
                    progress.status = TransferStatus::Failed(e.to_string());
                }
            }
        }
        let _ = std::fs::remove_file(&zip_path_str);
    });

    Ok(transfer_id.to_string())
}

#[tauri::command]
pub async fn get_transfer_progress(
    state: tauri::State<'_, Arc<AppState>>,
    transfer_id: String,
) -> Result<Option<TransferProgress>, String> {
    let tid = Uuid::parse_str(&transfer_id)
        .map_err(|e| format!("Invalid transfer ID: {}", e))?;
    let progress = state.transfer_progress.get(&tid).map(|p| p.clone());
    Ok(progress)
}

#[tauri::command]
pub async fn cancel_transfer(
    state: tauri::State<'_, Arc<AppState>>,
    transfer_id: String,
) -> Result<(), String> {
    let tid = Uuid::parse_str(&transfer_id)
        .map_err(|e| format!("Invalid transfer ID: {}", e))?;
    if let Some(mut progress) = state.transfer_progress.get_mut(&tid) {
        progress.status = TransferStatus::Cancelled;
    }
    Ok(())
}