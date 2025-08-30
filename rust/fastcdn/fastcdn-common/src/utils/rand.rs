use rand::Rng;

const HEX_CHARS: &str = "0123456789abcdef";
const HEX_CHARS_LENGTH: usize = HEX_CHARS.len();

const LETTER_CHARS: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LETTER_CHARS_LENGTH: usize = LETTER_CHARS.len();

/// 生成指定字符集内的随机字符
pub fn string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let charset = LETTER_CHARS.as_bytes();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..LETTER_CHARS_LENGTH);
            charset[idx] as char
        })
        .collect()
}

pub fn hex_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let charset = HEX_CHARS.as_bytes();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..HEX_CHARS_LENGTH);
            charset[idx] as char
        })
        .collect()
}
