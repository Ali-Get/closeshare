pub fn calculate_checksum(data: &[u8]) -> String {
    blake3::hash(data).to_hex().to_string()
}

pub fn verify_checksum(data: &[u8], expected: &str) -> bool {
    let actual = calculate_checksum(data);
    actual == expected
}