use crate::core::state::AppState;
use std::sync::Arc;

pub async fn start_transfer_listener(
    state: Arc<AppState>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = state.config.read().clone();
    let addr = format!("0.0.0.0:{}", config.transfer_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Transfer listener started on {}", addr);

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                tracing::info!("Incoming transfer connection from {}", addr);
                let state_clone = state.clone();
                let shared_folder = state_clone.config.read().shared_folder.clone();
                std::thread::spawn(move || {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(async {
                        if let Err(e) = crate::transfer::receiver::receive_file(
                            state_clone,
                            stream,
                            shared_folder,
                        ).await {
                            tracing::error!("Transfer error: {}", e);
                        }
                    });
                });
            }
            Err(e) => {
                tracing::error!("Failed to accept transfer connection: {}", e);
            }
        }
    }
}