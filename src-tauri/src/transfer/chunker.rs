#[derive(Debug, Clone)]
pub struct Chunk {
    pub index: u64,
    pub offset: u64,
    pub size: usize,
    pub total_chunks: u64,
}

pub struct Chunker {
    pub file_size: u64,
    pub chunk_size: usize,
    pub total_chunks: u64,
}

impl Chunker {
    pub fn new(file_size: u64, chunk_size: usize) -> Self {
        let total_chunks = (file_size + chunk_size as u64 - 1) / chunk_size as u64;
        Self {
            file_size,
            chunk_size,
            total_chunks,
        }
    }

    pub fn get_chunk(&self, index: u64) -> Option<Chunk> {
        if index >= self.total_chunks {
            return None;
        }
        let offset = index * self.chunk_size as u64;
        let remaining = self.file_size - offset;
        let size = std::cmp::min(remaining as usize, self.chunk_size);
        Some(Chunk {
            index,
            offset,
            size,
            total_chunks: self.total_chunks,
        })
    }
}

pub async fn read_chunk(
    file_path: &str,
    offset: u64,
    size: usize,
) -> Result<Vec<u8>, std::io::Error> {
    use std::io::{Read, Seek};
    let mut file = std::fs::File::open(file_path)?;
    file.seek(std::io::SeekFrom::Start(offset))?;
    let mut buffer = vec![0u8; size];
    file.read_exact(&mut buffer)?;
    Ok(buffer)
}

pub async fn write_chunk(
    file_path: &str,
    offset: u64,
    data: &[u8],
) -> Result<(), std::io::Error> {
    use std::io::{Seek, Write};
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)?;
    file.seek(std::io::SeekFrom::Start(offset))?;
    file.write_all(data)?;
    Ok(())
}