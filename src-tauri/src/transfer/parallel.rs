use tokio::sync::Semaphore;
use std::sync::Arc;

pub struct ParallelTransferManager {
    semaphore: Arc<Semaphore>,
    chunk_size: usize,
}

impl ParallelTransferManager {
    pub fn new(max_connections: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_connections)),
            chunk_size: 1024 * 1024,
        }
    }

    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    pub async fn acquire(&self) -> tokio::sync::OwnedSemaphorePermit {
        self.semaphore
            .clone()
            .acquire_owned()
            .await
            .expect("Semaphore closed")
    }
}

pub fn distribute_chunks(
    total_chunks: u64,
    num_connections: u64,
) -> Vec<Vec<u64>> {
    let mut distribution = vec![Vec::new(); num_connections as usize];
    let chunks_per_connection = total_chunks / num_connections;
    let remainder = total_chunks % num_connections;
    let mut chunk_index = 0u64;

    for (conn_idx, chunks) in distribution.iter_mut().enumerate() {
        let mut count = chunks_per_connection;
        if (conn_idx as u64) < remainder {
            count += 1;
        }
        for _ in 0..count {
            if chunk_index < total_chunks {
                chunks.push(chunk_index);
                chunk_index += 1;
            }
        }
    }

    distribution
}