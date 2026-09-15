use crate::core::config::Config;
use crate::core::state::AppState;
use crate::exchange::scanner;
use std::sync::Arc;

#[tauri::command]
pub async fn get_config(state: tauri::State<'_, Arc<AppState>>) -> Result<Config, String> {
    Ok(state.config.read().clone())
}

#[tauri::command]
pub async fn update_config(
    state: tauri::State<'_, Arc<AppState>>,
    config: Config,
) -> Result<(), String> {
    config.save().map_err(|e| e.to_string())?;
    let display_name = config.display_name.clone();
    let mut current = state.config.write();
    *current = config;
    let mut name = state.display_name.write();
    *name = display_name;
    Ok(())
}

#[tauri::command]
pub async fn get_shared_files(
    state: tauri::State<'_, Arc<AppState>>,
) -> Result<Vec<crate::protocol::message::FileMetadata>, String> {
    let config = state.config.read();
    let files = scanner::scan_directory(&config.shared_folder);
    let metadata: Vec<_> = files
        .into_iter()
        .map(|f| crate::protocol::message::FileMetadata {
            name: f.name,
            path: f.relative_path,
            size: f.size,
            is_directory: f.is_directory,
            modified: f.modified,
            checksum: f.checksum,
            mime_type: f.mime_type,
        })
        .collect();
    Ok(metadata)
}