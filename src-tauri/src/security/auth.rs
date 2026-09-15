pub fn verify_group_code(provided: &str, expected: &str) -> bool {
    if expected.is_empty() {
        return true;
    }
    provided == expected
}

pub fn generate_group_code() -> String {
    use rand::Rng;
    let code: u32 = rand::thread_rng().gen_range(1000..9999);
    format!("{:04}", code)
}

pub fn is_valid_group_code(code: &str) -> bool {
    code.len() >= 4 && code.chars().all(|c| c.is_ascii_digit())
}