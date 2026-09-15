mod protocol;
mod discovery;
mod exchange;
mod transfer;
mod security;
mod core;
mod commands;

use core::state::AppState;
use std::sync::Arc;
use tauri::Manager;

pub fn run() {
    let app_state = Arc::new(AppState::new());

    tauri::Builder::default()
        .manage(app_state.clone())
        .invoke_handler(tauri::generate_handler![
            commands::discovery_cmd::start_discovery,
            commands::discovery_cmd::stop_discovery,
            commands::discovery_cmd::get_peers,
            commands::transfer_cmd::send_file,
            commands::transfer_cmd::send_folder,
            commands::transfer_cmd::get_transfer_progress,
            commands::transfer_cmd::cancel_transfer,
            commands::settings_cmd::get_config,
            commands::settings_cmd::update_config,
            commands::settings_cmd::get_shared_files,
        ])
        .setup(|app| {
            let state: tauri::State<'_, Arc<AppState>> = app.state();
            let state_clone = state.inner().clone();
            let state_clone2 = state.inner().clone();

            // مستمع الاكتشاف
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    discovery::listener::start_listener(state_clone).await;
                });
            });

            // مرسل الاكتشاف (broadcaster)
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let config = state_clone2.config.read().clone();
                rt.block_on(async {
                    let _ = discovery::broadcaster::start_broadcaster(
                        state_clone2.device_id,
                        state_clone2.display_name.read().clone(),
                        state_clone2.hostname.clone(),
                        Arc::new(config),
                    ).await;
                });
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running CloseShare");
}