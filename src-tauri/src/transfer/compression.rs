use zstd::stream::{encode_all, decode_all};

const COMPRESSION_LEVEL: i32 = 1;

pub fn compress(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    encode_all(data, COMPRESSION_LEVEL)
}

pub fn decompress(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    decode_all(data)
}