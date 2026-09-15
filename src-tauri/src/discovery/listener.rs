use crate::core::state::AppState;
use crate::discovery::broadcaster;
use crate::discovery::peer::Peer;
use crate::protocol::codec;
use crate::protocol::message::ProtocolMessage;
use std::net::UdpSocket;
use std::sync::Arc;

pub async fn start_listener(state: Arc<AppState>) {
    let config = state.config.read().clone();
    let bind_addr = format!("0.0.0.0:{}", config.discovery_port);

    let socket = match UdpSocket::bind(&bind_addr) {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("Failed to bind discovery listener to {}: {}", bind_addr, e);
            return;
        }
    };

    tracing::info!("Discovery listener started on {}", bind_addr);

    let mut buffer = vec![0u8; 65535];

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((size, addr)) => {
                if let Ok(message) = codec::decode(&buffer[..size]) {
                    match &message {
                        ProtocolMessage::Discovery(d) if d.device_id == state.device_id => continue,
                        ProtocolMessage::DiscoveryResponse(d) if d.device_id == state.device_id => continue,
                        _ => {}
                    }
                    handle_message(&state, &socket, message, addr);
                }
            }
            Err(e) => {
                tracing::error!("Discovery receive error: {}", e);
                tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
            }
        }
    }
}

fn handle_message(
    state: &Arc<AppState>,
    socket: &UdpSocket,
    message: ProtocolMessage,
    addr: std::net::SocketAddr,
) {
    let config = state.config.read();

    match message {
        ProtocolMessage::Discovery(discovery) => {
            if !config.group_code.is_empty() && discovery.group_code != config.group_code {
                tracing::debug!(
                    "Ignored discovery from '{}' (different group code)",
                    discovery.display_name
                );
                return;
            }

            let peer = Peer::new(
                discovery.device_id,
                discovery.display_name.clone(),
                discovery.hostname.clone(),
                addr,
                discovery.transfer_port,
            );
            state.peers.insert(discovery.device_id, peer);

            tracing::info!("Discovered peer: {} at {}", discovery.display_name, addr);

            let local_addr = socket.local_addr().unwrap_or(addr);
            let _ = broadcaster::send_discovery_response(
                socket,
                state.device_id,
                &state.display_name.read(),
                &state.hostname,
                &config.group_code,
                local_addr,
                addr,
            );
        }

        ProtocolMessage::DiscoveryResponse(response) => {
            if !config.group_code.is_empty() && response.group_code != config.group_code {
                return;
            }

            let peer = Peer::new(
                response.device_id,
                response.display_name.clone(),
                response.hostname.clone(),
                response.socket_addr,
                0,
            );
            state.peers.insert(response.device_id, peer);

            tracing::info!(
                "Received discovery response from: {} at {}",
                response.display_name,
                response.socket_addr
            );
        }

        _ => {}
    }
}