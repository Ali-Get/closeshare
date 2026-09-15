use super::metadata::FileInfo;
use chrono::{DateTime, Utc};
use std::path::Path;
use walkdir::WalkDir;

pub fn scan_directory<P: AsRef<Path>>(root: P) -> Vec<FileInfo> {
    let root_path = root.as_ref();
    let mut files = Vec::new();

    for entry in WalkDir::new(root_path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with('.'))
            .unwrap_or(false)
        {
            continue;
        }

        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let relative_path = match path.strip_prefix(root_path) {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(_) => continue,
        };

        let modified: DateTime<Utc> = metadata
            .modified()
            .map(|t| t.into())
            .unwrap_or_else(|_| Utc::now());

        let mime_type = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();

        files.push(FileInfo::new(
            path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            relative_path,
            path.to_string_lossy().to_string(),
            metadata.len(),
            metadata.is_dir(),
            modified.to_rfc3339(),
        ));
    }

    files
}