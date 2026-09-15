use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub relative_path: String,
    pub absolute_path: String,
    pub size: u64,
    pub is_directory: bool,
    pub modified: String,
    pub mime_type: String,
    pub checksum: Option<String>,
}

impl FileInfo {
    pub fn new(
        name: String,
        relative_path: String,
        absolute_path: String,
        size: u64,
        is_directory: bool,
        modified: String,
    ) -> Self {
        Self {
            name,
            relative_path,
            absolute_path,
            size,
            is_directory,
            modified,
            mime_type: String::from("application/octet-stream"),
            checksum: None,
        }
    }
}