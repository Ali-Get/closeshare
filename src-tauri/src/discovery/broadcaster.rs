use crate::core::config::Config;
use crate::protocol::codec;
use crate::protocol::message::{DiscoveryMessage, ProtocolMessage};
use std::net::{SocketAddr, UdpSocket};
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

const BROADCAST_INTERVAL: Duration = Duration::from_secs(5);

pub async fn start_broadcaster(
    device_id: Uuid,
    display_name: String,
    hostname: String,
    config: Arc<Config>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    let broadcast_addr = SocketAddr::new(
        config.discovery_broadcast_addr.parse()?,
        config.discovery_port,
    );
    let discovery_msg = DiscoveryMessage {
        device_id,
        display_name,
        hostname,
        group_code: config.group_code.clone(),
        transfer_port: config.transfer_port,
    };
    let message = ProtocolMessage::Discovery(discovery_msg);
    let encoded = codec::encode(&message)?;
    tracing::info!("Starting discovery broadcaster on {}", broadcast_addr);
    loop {
        match socket.send_to(&encoded, broadcast_addr) {
            Ok(_) => tracing::trace!("Broadcast sent"),
            Err(e) => tracing::error!("Broadcast failed: {}", e),
        }
        tokio::time::sleep(BROADCAST_INTERVAL).await;
    }
}

pub fn send_discovery_response(
    socket: &UdpSocket,
    local_device_id: Uuid,
    local_display_name: &str,
    local_hostname: &str,
    group_code: &str,
    local_addr: SocketAddr,
    requester_addr: SocketAddr,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use crate::protocol::message::DiscoveryResponse;
    let response = ProtocolMessage::DiscoveryResponse(DiscoveryResponse {
        device_id: local_device_id,
        display_name: local_display_name.to_string(),
        hostname: local_hostname.to_string(),
        group_code: group_code.to_string(),
        socket_addr: local_addr,
    });
    codec::send_udp_message(socket, &response, requester_addr)?;
    Ok(())
}