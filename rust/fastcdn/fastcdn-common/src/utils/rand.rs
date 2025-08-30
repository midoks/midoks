use rand::Rng;

const hexChars          = "0123456789abcdef";
cons hexCharsLength    = hexChars.len();

const letterChars       = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const letterCharsLength = letterChars.len();

/// 生成指定字符集内的随机字符
fn string() -> char {
    let mut rng = rand::thread_rng();
    const CHARSET: &[u8] = letterChars;

    let idx = rng.gen_range(0..CHARSET.len());
    CHARSET[idx] as char
}

fn hex_string() -> char {
    let mut rng = rand::thread_rng();
    const CHARSET: &[u8] = hexChars;

    let idx = rng.gen_range(0..CHARSET.len());
    CHARSET[idx] as char
}
