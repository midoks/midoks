use md5;

use bcrypt::{DEFAULT_COST, hash, verify};

pub fn password_hash(password: &str) -> Result<String, bcrypt::BcryptError> {
    // 哈希密码，DEFAULT_COST 是计算成本（通常为12）
    let hashed = hash(password, DEFAULT_COST)?;
    // 将 `hashed` (格式类似于 "$2b$12$...") 存入数据库
    Ok(hashed)
}

pub fn verify_password_hash(
    stored_hash: &str,
    attempted_password: &str,
) -> Result<bool, bcrypt::BcryptError> {
    verify(attempted_password, stored_hash)
}

/// 计算字符串的MD5哈希值
///
/// # 参数
/// * `input` - 需要计算MD5的字符串
///
/// # 返回值
/// 返回32位小写的MD5哈希字符串
///
/// # 示例
/// ```
/// use fastcdn_common::utils::common::md5_string;
///
/// let hash = md5_string("hello world");
/// assert_eq!(hash, "5eb63bbbe01eeed093cb22bb8f5acdc3");
/// ```
pub fn md5_string(input: &str) -> String {
    let digest = md5::compute(input.as_bytes());
    format!("{:x}", digest)
}

/// 计算字节数组的MD5哈希值
///
/// # 参数
/// * `input` - 需要计算MD5的字节数组
///
/// # 返回值
/// 返回32位小写的MD5哈希字符串
///
/// # 示例
/// ```
/// use fastcdn_common::utils::common::md5_bytes;
///
/// let data = b"hello world";
/// let hash = md5_bytes(data);
/// assert_eq!(hash, "5eb63bbbe01eeed093cb22bb8f5acdc3");
/// ```
pub fn md5_bytes(input: &[u8]) -> String {
    let digest = md5::compute(input);
    format!("{:x}", digest)
}

/// 计算文件的MD5哈希值
///
/// # 参数
/// * `file_path` - 文件路径
///
/// # 返回值
/// 返回Result，成功时包含32位小写的MD5哈希字符串
///
/// # 示例
/// ```
/// use fastcdn_common::utils::common::md5_file;
///
/// match md5_file("/path/to/file.txt") {
///     Ok(hash) => println!("文件MD5: {}", hash),
///     Err(e) => println!("计算MD5失败: {}", e),
/// }
/// ```
pub fn md5_file(file_path: &str) -> Result<String, std::io::Error> {
    use std::fs;

    let contents = fs::read(file_path)?;
    let digest = md5::compute(&contents);
    Ok(format!("{:x}", digest))
}

/// 验证字符串的MD5哈希值
///
/// # 参数
/// * `input` - 原始字符串
/// * `expected_hash` - 期望的MD5哈希值
///
/// # 返回值
/// 返回布尔值，表示哈希值是否匹配
///
/// # 示例
/// ```
/// use fastcdn_common::utils::common::verify_md5_string;
///
/// let is_valid = verify_md5_string("hello world", "5eb63bbbe01eeed093cb22bb8f5acdc3");
/// assert!(is_valid);
/// ```
pub fn verify_md5_string(input: &str, expected_hash: &str) -> bool {
    let actual_hash = md5_string(input);
    actual_hash.eq_ignore_ascii_case(expected_hash)
}

/// 验证文件的MD5哈希值
///
/// # 参数
/// * `file_path` - 文件路径
/// * `expected_hash` - 期望的MD5哈希值
///
/// # 返回值
/// 返回Result，成功时包含布尔值表示哈希值是否匹配
///
/// # 示例
/// ```
/// use fastcdn_common::utils::common::verify_md5_file;
///
/// match verify_md5_file("/path/to/file.txt", "expected_hash") {
///     Ok(is_valid) => println!("文件验证结果: {}", is_valid),
///     Err(e) => println!("验证失败: {}", e),
/// }
/// ```
pub fn verify_md5_file(file_path: &str, expected_hash: &str) -> Result<bool, std::io::Error> {
    let actual_hash = md5_file(file_path)?;
    Ok(actual_hash.eq_ignore_ascii_case(expected_hash))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5_string() {
        assert_eq!(
            md5_string("hello world"),
            "5eb63bbbe01eeed093cb22bb8f5acdc3"
        );
        assert_eq!(md5_string(""), "d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(
            md5_string("The quick brown fox jumps over the lazy dog"),
            "9e107d9d372bb6826bd81d3542a419d6"
        );
    }

    #[test]
    fn test_md5_bytes() {
        assert_eq!(
            md5_bytes(b"hello world"),
            "5eb63bbbe01eeed093cb22bb8f5acdc3"
        );
        assert_eq!(md5_bytes(b""), "d41d8cd98f00b204e9800998ecf8427e");
    }

    #[test]
    fn test_verify_md5_string() {
        assert!(verify_md5_string(
            "hello world",
            "5eb63bbbe01eeed093cb22bb8f5acdc3"
        ));
        assert!(verify_md5_string(
            "hello world",
            "5EB63BBBE01EEED093CB22BB8F5ACDC3"
        )); // 大小写不敏感
        assert!(!verify_md5_string("hello world", "invalid_hash"));
    }

    #[test]
    fn test_md5_consistency() {
        let test_string = "test data for consistency";
        let hash1 = md5_string(test_string);
        let hash2 = md5_bytes(test_string.as_bytes());
        assert_eq!(hash1, hash2);
    }
}
