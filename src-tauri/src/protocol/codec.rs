use super::message::ProtocolMessage;
use bytes::{Buf, BufMut, BytesMut};
use thiserror::Error;

const MAX_MESSAGE_SIZE: usize = 100 * 1024 * 1024;

#[derive(Error, Debug)]
pub enum CodecError {
    #[error("Message too large: {0} bytes")]
    MessageTooLarge(usize),
    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Incomplete message")]
    Incomplete,
}

pub fn encode(message: &ProtocolMessage) -> Result<Vec<u8>, CodecError> {
    let payload = bincode::serialize(message)?;
    if payload.len() > MAX_MESSAGE_SIZE {
        return Err(CodecError::MessageTooLarge(payload.len()));
    }
    let mut buffer = BytesMut::with_capacity(4 + payload.len());
    buffer.put_u32(payload.len() as u32);
    buffer.put_slice(&payload);
    Ok(buffer.to_vec())
}

pub fn decode(data: &[u8]) -> Result<ProtocolMessage, CodecError> {
    if data.len() < 4 {
        return Err(CodecError::Incomplete);
    }
    let mut bytes = bytes::Bytes::copy_from_slice(data);
    let length = bytes.get_u32() as usize;
    if bytes.len() < length {
        return Err(CodecError::Incomplete);
    }
    let payload = bytes.slice(0..length);
    let message = bincode::deserialize(&payload)?;
    Ok(message)
}

pub async fn send_message(
    stream: &mut tokio::net::TcpStream,
    message: &ProtocolMessage,
) -> Result<(), CodecError> {
    use tokio::io::AsyncWriteExt;
    let encoded = encode(message)?;
    stream.write_all(&encoded).await?;
    stream.flush().await?;
    Ok(())
}

pub async fn receive_message(
    stream: &mut tokio::net::TcpStream,
) -> Result<ProtocolMessage, CodecError> {
    use tokio::io::AsyncReadExt;
    let mut length_buf = [0u8; 4];
    stream.read_exact(&mut length_buf).await?;
    let length = u32::from_be_bytes(length_buf) as usize;
    if length > MAX_MESSAGE_SIZE {
        return Err(CodecError::MessageTooLarge(length));
    }
    let mut payload = vec![0u8; length];
    stream.read_exact(&mut payload).await?;
    let message = bincode::deserialize(&payload)?;
    Ok(message)
}

pub fn send_udp_message(
    socket: &std::net::UdpSocket,
    message: &ProtocolMessage,
    addr: std::net::SocketAddr,
) -> Result<(), CodecError> {
    let encoded = encode(message)?;
    socket.send_to(&encoded, addr)?;
    Ok(())
}