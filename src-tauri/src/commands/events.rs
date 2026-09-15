use tauri::Window;

#[allow(dead_code)]
pub fn emit_peer_discovered(window: &Window, peer_name: &str) {
    let _ = window.emit("peer-discovered", serde_json::json!({
        "name": peer_name,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }));
}

#[allow(dead_code)]
pub fn emit_transfer_progress(window: &Window, transfer_id: &str, progress: f64) {
    let _ = window.emit("transfer-progress", serde_json::json!({
        "transferId": transfer_id,
        "progress": progress,
    }));
}

#[allow(dead_code)]
pub fn emit_transfer_complete(window: &Window, transfer_id: &str) {
    let _ = window.emit("transfer-complete", serde_json::json!({
        "transferId": transfer_id,
    }));
}