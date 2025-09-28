// 加密引擎实现
use crate::components::{ComponentResult, ComponentError};
#[cfg(feature = "crypto-advanced")]
use aes_gcm::{Aes256Gcm, Key, aead::{Aead, KeyInit, generic_array::GenericArray}};
#[cfg(feature = "crypto-advanced")]
use chacha20poly1305::{ChaCha20Poly1305, Key as ChaChaKey, Nonce as ChaChaNonce};
#[cfg(feature = "crypto-advanced")]
// use chacha20poly1305::aead::KeyInit as ChaChaKeyInit;
use std::collections::HashMap;

/// 加密引擎
pub struct EncryptionEngine {
    /// 加密算法映射
    algorithms: HashMap<String, Box<dyn EncryptionAlgorithm>>,
    /// 是否已初始化
    initialized: bool,
}

/// 加密算法 trait
pub trait EncryptionAlgorithm: Send + Sync {
    fn encrypt(&self, data: &[u8], key: &[u8]) -> ComponentResult<Vec<u8>>;
    fn decrypt(&self, encrypted_data: &[u8], key: &[u8]) -> ComponentResult<Vec<u8>>;
    fn name(&self) -> &str;
    fn key_size(&self) -> usize;
    fn nonce_size(&self) -> usize;
}

/// AES-GCM 算法
#[cfg(feature = "crypto-advanced")]
pub struct AesGcmAlgorithm;

#[cfg(feature = "crypto-advanced")]
impl EncryptionAlgorithm for AesGcmAlgorithm {
    fn encrypt(&self, data: &[u8], key: &[u8]) -> ComponentResult<Vec<u8>> {
        if key.len() != 32 {
            return Err(ComponentError::CryptographyError(
                "Invalid key length for AES-GCM (must be 32 bytes)".to_string()
            ));
        }

        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        let nonce = self.generate_nonce();
        
        let ciphertext = cipher.encrypt(&nonce, data)
            .map_err(|e| ComponentError::CryptographyError(format!("Encryption failed: {}", e)))?;

        // 将nonce和密文组合
        let mut result = Vec::new();
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);
        
        Ok(result)
    }

    fn decrypt(&self, encrypted_data: &[u8], key: &[u8]) -> ComponentResult<Vec<u8>> {
        if key.len() != 32 {
            return Err(ComponentError::CryptographyError(
                "Invalid key length for AES-GCM (must be 32 bytes)".to_string()
            ));
        }

        if encrypted_data.len() < 12 {
            return Err(ComponentError::CryptographyError(
                "Invalid encrypted data length".to_string()
            ));
        }

        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        let nonce = GenericArray::from_slice(&encrypted_data[..12]);
        let ciphertext = &encrypted_data[12..];

        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| ComponentError::CryptographyError(format!("Decryption failed: {}", e)))?;

        Ok(plaintext)
    }

    fn name(&self) -> &str {
        "aes-gcm"
    }

    fn key_size(&self) -> usize {
        32
    }

    fn nonce_size(&self) -> usize {
        12
    }
}

#[cfg(feature = "crypto-advanced")]
impl AesGcmAlgorithm {
    fn generate_nonce(&self) -> GenericArray<u8, aes_gcm::aead::generic_array::typenum::U12> {
        use rand::Rng;
        let mut rng = rand::rng();
        let mut nonce = [0u8; 12];
        rng.fill(&mut nonce);
        GenericArray::from_slice(&nonce).clone()
    }
}

/// ChaCha20-Poly1305 算法
#[cfg(feature = "crypto-advanced")]
pub struct ChaCha20Poly1305Algorithm;

#[cfg(feature = "crypto-advanced")]
impl EncryptionAlgorithm for ChaCha20Poly1305Algorithm {
    fn encrypt(&self, data: &[u8], key: &[u8]) -> ComponentResult<Vec<u8>> {
        if key.len() != 32 {
            return Err(ComponentError::CryptographyError(
                "Invalid key length for ChaCha20-Poly1305 (must be 32 bytes)".to_string()
            ));
        }

        let cipher = ChaCha20Poly1305::new(ChaChaKey::from_slice(key));
        let nonce = self.generate_nonce();
        
        let ciphertext = cipher.encrypt(&nonce, data)
            .map_err(|e| ComponentError::CryptographyError(format!("Encryption failed: {}", e)))?;

        // 将nonce和密文组合
        let mut result = Vec::new();
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);
        
        Ok(result)
    }

    fn decrypt(&self, encrypted_data: &[u8], key: &[u8]) -> ComponentResult<Vec<u8>> {
        if key.len() != 32 {
            return Err(ComponentError::CryptographyError(
                "Invalid key length for ChaCha20-Poly1305 (must be 32 bytes)".to_string()
            ));
        }

        if encrypted_data.len() < 12 {
            return Err(ComponentError::CryptographyError(
                "Invalid encrypted data length".to_string()
            ));
        }

        let cipher = ChaCha20Poly1305::new(ChaChaKey::from_slice(key));
        let nonce = ChaChaNonce::from_slice(&encrypted_data[..12]);
        let ciphertext = &encrypted_data[12..];

        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| ComponentError::CryptographyError(format!("Decryption failed: {}", e)))?;

        Ok(plaintext)
    }

    fn name(&self) -> &str {
        "chacha20-poly1305"
    }

    fn key_size(&self) -> usize {
        32
    }

    fn nonce_size(&self) -> usize {
        12
    }
}

#[cfg(feature = "crypto-advanced")]
impl ChaCha20Poly1305Algorithm {
    fn generate_nonce(&self) -> ChaChaNonce {
        use rand::Rng;
        let mut rng = rand::rng();
        let mut nonce = [0u8; 12];
        rng.fill(&mut nonce);
        ChaChaNonce::from_slice(&nonce).clone()
    }
}

impl EncryptionEngine {
    /// 创建新的加密引擎
    pub fn new() -> Self {
        Self {
            algorithms: HashMap::new(),
            initialized: false,
        }
    }

    /// 初始化加密引擎
    pub async fn initialize(&mut self) -> ComponentResult<()> {
        // 注册默认加密算法
        #[cfg(feature = "crypto-advanced")]
        {
            self.register_algorithm("aes-gcm", Box::new(AesGcmAlgorithm));
            self.register_algorithm("chacha20-poly1305", Box::new(ChaCha20Poly1305Algorithm));
        }

        self.initialized = true;
        Ok(())
    }

    /// 注册加密算法
    pub fn register_algorithm(&mut self, name: &str, algorithm: Box<dyn EncryptionAlgorithm>) {
        self.algorithms.insert(name.to_string(), algorithm);
    }

    /// 获取加密算法
    fn get_algorithm(&self, name: &str) -> ComponentResult<&dyn EncryptionAlgorithm> {
        self.algorithms.get(name)
            .map(|alg| alg.as_ref())
            .ok_or_else(|| ComponentError::CryptographyError(format!("Encryption algorithm '{}' not found", name)))
    }

    /// 加密数据
    pub fn encrypt(&self, data: &[u8], key: &[u8], algorithm: &str) -> ComponentResult<Vec<u8>> {
        let alg = self.get_algorithm(algorithm)?;
        alg.encrypt(data, key)
    }

    /// 解密数据
    pub fn decrypt(&self, encrypted_data: &[u8], key: &[u8], algorithm: &str) -> ComponentResult<Vec<u8>> {
        let alg = self.get_algorithm(algorithm)?;
        alg.decrypt(encrypted_data, key)
    }

    /// 生成随机密钥
    pub fn generate_key(&self, algorithm: &str) -> ComponentResult<Vec<u8>> {
        let alg = self.get_algorithm(algorithm)?;
        let key_size = alg.key_size();
        
        use rand::Rng;
        let mut rng = rand::rng();
        let mut key = vec![0u8; key_size];
        rng.fill(&mut key[..]);
        
        Ok(key)
    }

    /// 生成随机nonce
    pub fn generate_nonce(&self, algorithm: &str) -> ComponentResult<Vec<u8>> {
        let alg = self.get_algorithm(algorithm)?;
        let nonce_size = alg.nonce_size();
        
        use rand::Rng;
        let mut rng = rand::rng();
        let mut nonce = vec![0u8; nonce_size];
        rng.fill(&mut nonce[..]);
        
        Ok(nonce)
    }

    /// 获取支持的算法列表
    pub fn get_supported_algorithms(&self) -> Vec<String> {
        self.algorithms.keys().cloned().collect()
    }

    /// 检查是否已初始化
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// 验证密钥格式
    pub fn validate_key_format(&self, key: &[u8], algorithm: &str) -> ComponentResult<bool> {
        let alg = self.get_algorithm(algorithm)?;
        Ok(key.len() == alg.key_size())
    }

    /// 验证加密数据格式
    pub fn validate_encrypted_data_format(&self, encrypted_data: &[u8], algorithm: &str) -> ComponentResult<bool> {
        let alg = self.get_algorithm(algorithm)?;
        Ok(encrypted_data.len() >= alg.nonce_size())
    }

    /// 从密码派生密钥
    pub fn derive_key_from_password(&self, password: &str, salt: &[u8], algorithm: &str) -> ComponentResult<Vec<u8>> {
        let alg = self.get_algorithm(algorithm)?;
        let key_size = alg.key_size();
        
        // 使用PBKDF2派生密钥
        use pbkdf2::pbkdf2_hmac;
        use sha2::Sha256;
        
        let mut key = vec![0u8; key_size];
        pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, 10000, &mut key);
        
        Ok(key)
    }

    /// 安全比较两个字节数组
    pub fn secure_compare(&self, a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let mut result = 0u8;
        for (x, y) in a.iter().zip(b.iter()) {
            result |= x ^ y;
        }
        result == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_encryption_engine_initialization() {
        let mut engine = EncryptionEngine::new();
        assert!(!engine.is_initialized());

        engine.initialize().await.unwrap();
        assert!(engine.is_initialized());
    }

    #[tokio::test]
    async fn test_aes_gcm_encryption() {
        let mut engine = EncryptionEngine::new();
        engine.initialize().await.unwrap();

        let data = b"Hello, Blockchain!";
        let key = engine.generate_key("aes-gcm").unwrap();

        let encrypted = engine.encrypt(data, &key, "aes-gcm").unwrap();
        let decrypted = engine.decrypt(&encrypted, &key, "aes-gcm").unwrap();

        assert_eq!(data, decrypted.as_slice());
    }

    #[tokio::test]
    async fn test_chacha20_poly1305_encryption() {
        let mut engine = EncryptionEngine::new();
        engine.initialize().await.unwrap();

        let data = b"Hello, Blockchain!";
        let key = engine.generate_key("chacha20-poly1305").unwrap();

        let encrypted = engine.encrypt(data, &key, "chacha20-poly1305").unwrap();
        let decrypted = engine.decrypt(&encrypted, &key, "chacha20-poly1305").unwrap();

        assert_eq!(data, decrypted.as_slice());
    }

    #[tokio::test]
    async fn test_key_generation() {
        let mut engine = EncryptionEngine::new();
        engine.initialize().await.unwrap();

        let key1 = engine.generate_key("aes-gcm").unwrap();
        let key2 = engine.generate_key("aes-gcm").unwrap();

        assert_eq!(key1.len(), 32);
        assert_eq!(key2.len(), 32);
        assert_ne!(key1, key2);
    }

    #[tokio::test]
    async fn test_nonce_generation() {
        let mut engine = EncryptionEngine::new();
        engine.initialize().await.unwrap();

        let nonce1 = engine.generate_nonce("aes-gcm").unwrap();
        let nonce2 = engine.generate_nonce("aes-gcm").unwrap();

        assert_eq!(nonce1.len(), 12);
        assert_eq!(nonce2.len(), 12);
        assert_ne!(nonce1, nonce2);
    }

    #[tokio::test]
    async fn test_password_derivation() {
        let mut engine = EncryptionEngine::new();
        engine.initialize().await.unwrap();

        let password = "test_password";
        let salt = b"test_salt";
        
        let key1 = engine.derive_key_from_password(password, salt, "aes-gcm").unwrap();
        let key2 = engine.derive_key_from_password(password, salt, "aes-gcm").unwrap();

        assert_eq!(key1.len(), 32);
        assert_eq!(key1, key2);
    }

    #[tokio::test]
    async fn test_secure_compare() {
        let engine = EncryptionEngine::new();

        let a = b"test_data";
        let b = b"test_data";
        let c = b"different_data";

        assert!(engine.secure_compare(a, b));
        assert!(!engine.secure_compare(a, c));
        assert!(!engine.secure_compare(a, b"shorter"));
    }

    #[tokio::test]
    async fn test_encryption_with_wrong_key() {
        let mut engine = EncryptionEngine::new();
        engine.initialize().await.unwrap();

        let data = b"Hello, Blockchain!";
        let key1 = engine.generate_key("aes-gcm").unwrap();
        let key2 = engine.generate_key("aes-gcm").unwrap();

        let encrypted = engine.encrypt(data, &key1, "aes-gcm").unwrap();
        let result = engine.decrypt(&encrypted, &key2, "aes-gcm");

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_supported_algorithms() {
        let mut engine = EncryptionEngine::new();
        engine.initialize().await.unwrap();

        let algorithms = engine.get_supported_algorithms();
        assert!(algorithms.contains(&"aes-gcm".to_string()));
        assert!(algorithms.contains(&"chacha20-poly1305".to_string()));
    }
}
