use crate::core::state::AppState;
use crate::discovery::broadcaster;
use std::sync::Arc;

#[tauri::command]
pub async fn start_discovery(state: tauri::State<'_, Arc<AppState>>) -> Result<(), String> {
    let state = state.inner().clone();
    let config = state.config.read().clone();
    let device_id = state.device_id;
    let display_name = state.display_name.read().clone();
    let hostname = state.hostname.clone();
    let config = Arc::new(config);

    tokio::spawn(async move {
        if let Err(e) = broadcaster::start_broadcaster(
            device_id,
            display_name,
            hostname,
            config,
        ).await {
            tracing::error!("Discovery broadcaster failed: {}", e);
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn stop_discovery(state: tauri::State<'_, Arc<AppState>>) -> Result<(), String> {
    tracing::info!("Discovery stop requested");
    Ok(())
}

#[tauri::command]
pub async fn get_peers(state: tauri::State<'_, Arc<AppState>>) -> Result<Vec<crate::discovery::peer::Peer>, String> {
    let peers: Vec<_> = state.peers.iter().map(|entry| entry.value().clone()).collect();
    Ok(peers)
}