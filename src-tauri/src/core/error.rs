use thiserror::Error;

#[derive(Error, Debug)]
pub enum CloseShareError {
    #[error("Network error: {0}")]
    Network(#[from] std::io::Error),
    #[error("Protocol error: {0}")]
    Protocol(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Security error: {0}")]
    Security(String),
    #[error("Transfer error: {0}")]
    Transfer(String),
    #[error("Peer not found: {0}")]
    PeerNotFound(String),
    #[error("Group code mismatch")]
    GroupCodeMismatch,
    #[error("File not found: {0}")]
    FileNotFound(String),
}

impl serde::Serialize for CloseShareError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}