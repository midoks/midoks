use aes::cipher::{BlockEncrypt, KeyInit};
use aes::{Aes128, Aes192, Aes256};
use generic_array::GenericArray;
use std::error::Error;
use std::fmt;

/// AES 加密错误类型
#[derive(Debug)]
pub enum AesError {
    InvalidKeyLength,
    InvalidIvLength,
    EncryptionFailed,
    DecryptionFailed,
}

impl fmt::Display for AesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AesError::InvalidKeyLength => write!(f, "Invalid key length"),
            AesError::InvalidIvLength => write!(f, "Invalid IV length"),
            AesError::EncryptionFailed => write!(f, "Encryption failed"),
            AesError::DecryptionFailed => write!(f, "Decryption failed"),
        }
    }
}

impl Error for AesError {}

/// 调整IV长度到16字节
/// 如果IV长度大于16字节，截取前16字节
/// 如果IV长度小于16字节，用零填充到16字节
fn adjust_iv_length(iv: &[u8]) -> [u8; 16] {
    let mut adjusted_iv = [0u8; 16];
    let copy_len = std::cmp::min(iv.len(), 16);
    adjusted_iv[..copy_len].copy_from_slice(&iv[..copy_len]);
    adjusted_iv
}

/// AES-128-CFB 加密
pub fn aes128_cfb_encrypt(key: &[u8], iv: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, AesError> {
    if key.len() != 16 {
        return Err(AesError::InvalidKeyLength);
    }
    let adjusted_iv = adjust_iv_length(iv);
    let cipher = Aes128::new(GenericArray::from_slice(key));
    let mut feedback = *GenericArray::from_slice(&adjusted_iv);
    let mut ciphertext = Vec::with_capacity(plaintext.len());

    for &byte in plaintext {
        cipher.encrypt_block(&mut feedback);
        let encrypted_byte = byte ^ feedback[0];
        ciphertext.push(encrypted_byte);

        // 左移 feedback 并添加新的密文字节
        for i in 0..15 {
            feedback[i] = feedback[i + 1];
        }
        feedback[15] = encrypted_byte;
    }

    Ok(ciphertext)
}

/// AES-128-CFB 解密
pub fn aes128_cfb_decrypt(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, AesError> {
    if key.len() != 16 {
        return Err(AesError::InvalidKeyLength);
    }
    let adjusted_iv = adjust_iv_length(iv);
    let cipher = Aes128::new(GenericArray::from_slice(key));
    let mut feedback = *GenericArray::from_slice(&adjusted_iv);
    let mut plaintext = Vec::with_capacity(ciphertext.len());

    for &byte in ciphertext {
        cipher.encrypt_block(&mut feedback);
        let decrypted_byte = byte ^ feedback[0];
        plaintext.push(decrypted_byte);

        // 左移 feedback 并添加新的密文字节
        for i in 0..15 {
            feedback[i] = feedback[i + 1];
        }
        feedback[15] = byte;
    }

    Ok(plaintext)
}

/// AES-192-CFB 加密
pub fn aes192_cfb_encrypt(key: &[u8], iv: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, AesError> {
    if key.len() != 24 {
        return Err(AesError::InvalidKeyLength);
    }
    let adjusted_iv = adjust_iv_length(iv);
    let cipher = Aes192::new(GenericArray::from_slice(key));
    let mut feedback = *GenericArray::from_slice(&adjusted_iv);
    let mut ciphertext = Vec::with_capacity(plaintext.len());

    for &byte in plaintext {
        cipher.encrypt_block(&mut feedback);
        let encrypted_byte = byte ^ feedback[0];
        ciphertext.push(encrypted_byte);

        // 左移 feedback 并添加新的密文字节
        for i in 0..15 {
            feedback[i] = feedback[i + 1];
        }
        feedback[15] = encrypted_byte;
    }

    Ok(ciphertext)
}

/// AES-192-CFB 解密
pub fn aes192_cfb_decrypt(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, AesError> {
    if key.len() != 24 {
        return Err(AesError::InvalidKeyLength);
    }
    let adjusted_iv = adjust_iv_length(iv);
    let cipher = Aes192::new(GenericArray::from_slice(key));
    let mut feedback = *GenericArray::from_slice(&adjusted_iv);
    let mut plaintext = Vec::with_capacity(ciphertext.len());

    for &byte in ciphertext {
        cipher.encrypt_block(&mut feedback);
        let decrypted_byte = byte ^ feedback[0];
        plaintext.push(decrypted_byte);

        // 左移 feedback 并添加新的密文字节
        for i in 0..15 {
            feedback[i] = feedback[i + 1];
        }
        feedback[15] = byte;
    }

    Ok(plaintext)
}

/// AES-256-CFB 加密
pub fn aes256_cfb_encrypt(key: &[u8], iv: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, AesError> {
    if key.len() != 32 {
        return Err(AesError::InvalidKeyLength);
    }
    let adjusted_iv = adjust_iv_length(iv);
    let cipher = Aes256::new(GenericArray::from_slice(key));
    let mut feedback = *GenericArray::from_slice(&adjusted_iv);
    let mut ciphertext = Vec::with_capacity(plaintext.len());

    for &byte in plaintext {
        cipher.encrypt_block(&mut feedback);
        let encrypted_byte = byte ^ feedback[0];
        ciphertext.push(encrypted_byte);

        // 左移 feedback 并添加新的密文字节
        for i in 0..15 {
            feedback[i] = feedback[i + 1];
        }
        feedback[15] = encrypted_byte;
    }

    Ok(ciphertext)
}

/// AES-256-CFB 解密
pub fn aes256_cfb_decrypt(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, AesError> {
    if key.len() != 32 {
        return Err(AesError::InvalidKeyLength);
    }
    let adjusted_iv = adjust_iv_length(iv);
    let cipher = Aes256::new(GenericArray::from_slice(key));
    let mut feedback = *GenericArray::from_slice(&adjusted_iv);
    let mut plaintext = Vec::with_capacity(ciphertext.len());

    for &byte in ciphertext {
        cipher.encrypt_block(&mut feedback);
        let decrypted_byte = byte ^ feedback[0];
        plaintext.push(decrypted_byte);

        // 左移 feedback 并添加新的密文字节
        for i in 0..15 {
            feedback[i] = feedback[i + 1];
        }
        feedback[15] = byte;
    }

    Ok(plaintext)
}

/// AES CFB 加密器结构体
pub struct AesCfbCipher {
    key_size: usize,
}

impl AesCfbCipher {
    /// 创建新的 AES CFB 加密器
    pub fn new(key_size: usize) -> Result<Self, AesError> {
        match key_size {
            128 | 192 | 256 => Ok(AesCfbCipher { key_size }),
            _ => Err(AesError::InvalidKeyLength),
        }
    }

    /// 加密数据
    pub fn encrypt(&self, key: &[u8], iv: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, AesError> {
        match self.key_size {
            128 => aes128_cfb_encrypt(key, iv, plaintext),
            192 => aes192_cfb_encrypt(key, iv, plaintext),
            256 => aes256_cfb_encrypt(key, iv, plaintext),
            _ => Err(AesError::InvalidKeyLength),
        }
    }

    /// 解密数据
    pub fn decrypt(&self, key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, AesError> {
        match self.key_size {
            128 => aes128_cfb_decrypt(key, iv, ciphertext),
            192 => aes192_cfb_decrypt(key, iv, ciphertext),
            256 => aes256_cfb_decrypt(key, iv, ciphertext),
            _ => Err(AesError::InvalidKeyLength),
        }
    }
}

// cargo test --package fastcdn-common utils::aes
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes128_cfb() {
        let key = b"1234567890123456"; // 16 bytes
        let iv = b"abcdefghijklmnop"; // 16 bytes
        let plaintext = b"Hello, World! This is a test message.";

        let ciphertext = aes128_cfb_encrypt(key, iv, plaintext).unwrap();
        let decrypted = aes128_cfb_decrypt(key, iv, &ciphertext).unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }

    #[test]
    fn test_aes192_cfb() {
        let key = b"123456789012345678901234"; // 24 bytes
        let iv = b"abcdefghijklmnop"; // 16 bytes
        let plaintext = b"Hello, World! This is a test message.";

        let ciphertext = aes192_cfb_encrypt(key, iv, plaintext).unwrap();
        let decrypted = aes192_cfb_decrypt(key, iv, &ciphertext).unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }

    #[test]
    fn test_aes256_cfb() {
        let key = b"12345678901234567890123456789012"; // 32 bytes
        let iv = b"abcdefghijklmnop"; // 16 bytes
        let plaintext = b"Hello, World! This is a test message.";

        let ciphertext = aes256_cfb_encrypt(key, iv, plaintext).unwrap();
        let decrypted = aes256_cfb_decrypt(key, iv, &ciphertext).unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }

    #[test]
    fn test_aes_cfb_cipher() {
        let cipher = AesCfbCipher::new(256).unwrap();
        let key = b"12345678901234567890123456789012"; // 32 bytes
        let iv = b"abcdefghijklmnop"; // 16 bytes
        let plaintext = b"Hello, World! This is a test message.";

        let ciphertext = cipher.encrypt(key, iv, plaintext).unwrap();
        let decrypted = cipher.decrypt(key, iv, &ciphertext).unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }

    #[test]
    fn test_invalid_key_length() {
        let key = b"short"; // 5 bytes, invalid
        let iv = b"abcdefghijklmnop"; // 16 bytes
        let plaintext = b"Hello, World!";

        let result = aes128_cfb_encrypt(key, iv, plaintext);
        assert!(matches!(result, Err(AesError::InvalidKeyLength)));
    }

    #[test]
    fn test_iv_length_adjustment() {
        let key = b"1234567890123456"; // 16 bytes
        let short_iv = b"short"; // 5 bytes, will be padded
        let long_iv = b"this_is_a_very_long_iv_that_exceeds_16_bytes"; // >16 bytes, will be truncated
        let plaintext = b"Hello, World!";

        // Test with short IV (should be padded)
        let result1 = aes128_cfb_encrypt(key, short_iv, plaintext);
        assert!(result1.is_ok());

        // Test with long IV (should be truncated)
        let result2 = aes128_cfb_encrypt(key, long_iv, plaintext);
        assert!(result2.is_ok());

        // Both should produce valid ciphertext
        let ciphertext1 = result1.unwrap();
        let ciphertext2 = result2.unwrap();
        assert!(!ciphertext1.is_empty());
        assert!(!ciphertext2.is_empty());
    }
}
