// 签名引擎实现
use crate::components::{ComponentResult, ComponentError};
use secp256k1::{Secp256k1, SecretKey, PublicKey, Message};
use secp256k1::ecdsa::Signature;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature as Ed25519Signature, Signer, Verifier};
use std::collections::HashMap;

/// 签名引擎
pub struct SignatureEngine {
    /// 签名算法映射
    algorithms: HashMap<String, Box<dyn SignatureAlgorithm>>,
    /// 是否已初始化
    initialized: bool,
}

/// 签名算法 trait
pub trait SignatureAlgorithm: Send + Sync {
    fn sign(&self, data: &[u8], private_key: &[u8]) -> ComponentResult<Vec<u8>>;
    fn verify(&self, data: &[u8], signature: &[u8], public_key: &[u8]) -> ComponentResult<bool>;
    fn name(&self) -> &str;
    fn key_size(&self) -> usize;
    fn signature_size(&self) -> usize;
}

/// ECDSA (secp256k1) 算法
pub struct EcdsaAlgorithm {
    secp: Secp256k1<secp256k1::All>,
}

impl EcdsaAlgorithm {
    pub fn new() -> Self {
        Self {
            secp: Secp256k1::new(),
        }
    }
}

impl SignatureAlgorithm for EcdsaAlgorithm {
    fn sign(&self, data: &[u8], private_key: &[u8]) -> ComponentResult<Vec<u8>> {
        if private_key.len() != 32 {
            return Err(ComponentError::CryptographyError(
                "Invalid private key length for ECDSA".to_string()
            ));
        }

        let secret_key = SecretKey::from_byte_array(private_key.try_into()
            .map_err(|_| ComponentError::CryptographyError("Invalid private key length".to_string()))?)
            .map_err(|e| ComponentError::CryptographyError(format!("Invalid private key: {}", e)))?;

        let message = Message::from_digest(data.try_into()
            .map_err(|_| ComponentError::CryptographyError("Invalid message length".to_string()))?);

        let signature = self.secp.sign_ecdsa(message, &secret_key);
        Ok(signature.serialize_der().to_vec())
    }

    fn verify(&self, data: &[u8], signature: &[u8], public_key: &[u8]) -> ComponentResult<bool> {
        if public_key.len() != 33 && public_key.len() != 65 {
            return Err(ComponentError::CryptographyError(
                "Invalid public key length for ECDSA".to_string()
            ));
        }

        let public_key = PublicKey::from_slice(public_key)
            .map_err(|e| ComponentError::CryptographyError(format!("Invalid public key: {}", e)))?;

        let signature = Signature::from_der(signature)
            .map_err(|e| ComponentError::CryptographyError(format!("Invalid signature: {}", e)))?;

        let message = Message::from_digest(data.try_into()
            .map_err(|_| ComponentError::CryptographyError("Invalid message length".to_string()))?);

        Ok(self.secp.verify_ecdsa(message, &signature, &public_key).is_ok())
    }

    fn name(&self) -> &str {
        "ecdsa"
    }

    fn key_size(&self) -> usize {
        32
    }

    fn signature_size(&self) -> usize {
        64
    }
}

/// Ed25519 算法
pub struct Ed25519Algorithm;

impl SignatureAlgorithm for Ed25519Algorithm {
    fn sign(&self, data: &[u8], private_key: &[u8]) -> ComponentResult<Vec<u8>> {
        if private_key.len() != 32 {
            return Err(ComponentError::CryptographyError(
                "Invalid private key length for Ed25519".to_string()
            ));
        }

        let signing_key = SigningKey::from_bytes(
            private_key.try_into()
                .map_err(|_| ComponentError::CryptographyError("Invalid private key format".to_string()))?
        );

        let signature = signing_key.sign(data);
        Ok(signature.to_bytes().to_vec())
    }

    fn verify(&self, data: &[u8], signature: &[u8], public_key: &[u8]) -> ComponentResult<bool> {
        if public_key.len() != 32 {
            return Err(ComponentError::CryptographyError(
                "Invalid public key length for Ed25519".to_string()
            ));
        }

        if signature.len() != 64 {
            return Err(ComponentError::CryptographyError(
                "Invalid signature length for Ed25519".to_string()
            ));
        }

        let verifying_key = VerifyingKey::from_bytes(
            public_key.try_into()
                .map_err(|_| ComponentError::CryptographyError("Invalid public key format".to_string()))?
        ).map_err(|e| ComponentError::CryptographyError(format!("Invalid public key: {}", e)))?;

        let signature = Ed25519Signature::from_bytes(
            signature.try_into()
                .map_err(|_| ComponentError::CryptographyError("Invalid signature format".to_string()))?
        );

        Ok(verifying_key.verify(data, &signature).is_ok())
    }

    fn name(&self) -> &str {
        "ed25519"
    }

    fn key_size(&self) -> usize {
        32
    }

    fn signature_size(&self) -> usize {
        64
    }
}

impl SignatureEngine {
    /// 创建新的签名引擎
    pub fn new() -> Self {
        Self {
            algorithms: HashMap::new(),
            initialized: false,
        }
    }

    /// 初始化签名引擎
    pub async fn initialize(&mut self) -> ComponentResult<()> {
        // 注册默认签名算法
        self.register_algorithm("ecdsa", Box::new(EcdsaAlgorithm::new()));
        self.register_algorithm("ed25519", Box::new(Ed25519Algorithm));

        self.initialized = true;
        Ok(())
    }

    /// 注册签名算法
    pub fn register_algorithm(&mut self, name: &str, algorithm: Box<dyn SignatureAlgorithm>) {
        self.algorithms.insert(name.to_string(), algorithm);
    }

    /// 获取签名算法
    fn get_algorithm(&self, name: &str) -> ComponentResult<&dyn SignatureAlgorithm> {
        self.algorithms.get(name)
            .map(|alg| alg.as_ref())
            .ok_or_else(|| ComponentError::CryptographyError(format!("Signature algorithm '{}' not found", name)))
    }

    /// 签名数据
    pub fn sign(&self, data: &[u8], private_key: &[u8], algorithm: &str) -> ComponentResult<Vec<u8>> {
        let alg = self.get_algorithm(algorithm)?;
        alg.sign(data, private_key)
    }

    /// 验证签名
    pub fn verify(&self, data: &[u8], signature: &[u8], public_key: &[u8], algorithm: &str) -> ComponentResult<bool> {
        let alg = self.get_algorithm(algorithm)?;
        alg.verify(data, signature, public_key)
    }

    /// 生成密钥对
    pub fn generate_keypair(&self, algorithm: &str) -> ComponentResult<(Vec<u8>, Vec<u8>)> {
        match algorithm {
            "ecdsa" => self.generate_ecdsa_keypair(),
            "ed25519" => self.generate_ed25519_keypair(),
            _ => Err(ComponentError::CryptographyError(format!("Unsupported algorithm: {}", algorithm))),
        }
    }

    /// 生成ECDSA密钥对
    fn generate_ecdsa_keypair(&self) -> ComponentResult<(Vec<u8>, Vec<u8>)> {
        use rand::Rng;
        let mut rng = rand::rng();
        let mut private_key_bytes = [0u8; 32];
        rng.fill(&mut private_key_bytes);

        let secret_key = SecretKey::from_byte_array(private_key_bytes)
            .map_err(|e| ComponentError::CryptographyError(format!("Failed to create secret key: {}", e)))?;

        let secp = Secp256k1::new();
        let public_key = secret_key.public_key(&secp);

        Ok((
            secret_key.secret_bytes().to_vec(),
            public_key.serialize().to_vec(),
        ))
    }

    /// 生成Ed25519密钥对
    fn generate_ed25519_keypair(&self) -> ComponentResult<(Vec<u8>, Vec<u8>)> {
        use ed25519_dalek::SigningKey;
        use rand::Rng;
        let mut rng = rand::rng();
        let mut key_bytes = [0u8; 32];
        rng.fill(&mut key_bytes);
        let signing_key = SigningKey::from_bytes(&key_bytes);
        let verifying_key = signing_key.verifying_key();

        Ok((
            signing_key.to_bytes().to_vec(),
            verifying_key.to_bytes().to_vec(),
        ))
    }

    /// 从私钥推导公钥
    pub fn derive_public_key(&self, private_key: &[u8], algorithm: &str) -> ComponentResult<Vec<u8>> {
        match algorithm {
            "ecdsa" => self.derive_ecdsa_public_key(private_key),
            "ed25519" => self.derive_ed25519_public_key(private_key),
            _ => Err(ComponentError::CryptographyError(format!("Unsupported algorithm: {}", algorithm))),
        }
    }

    /// 从ECDSA私钥推导公钥
    fn derive_ecdsa_public_key(&self, private_key: &[u8]) -> ComponentResult<Vec<u8>> {
        if private_key.len() != 32 {
            return Err(ComponentError::CryptographyError(
                "Invalid private key length for ECDSA".to_string()
            ));
        }

        let secret_key = SecretKey::from_byte_array(private_key.try_into()
            .map_err(|_| ComponentError::CryptographyError("Invalid private key length".to_string()))?)
            .map_err(|e| ComponentError::CryptographyError(format!("Invalid private key: {}", e)))?;

        let secp = Secp256k1::new();
        let public_key = secret_key.public_key(&secp);

        Ok(public_key.serialize().to_vec())
    }

    /// 从Ed25519私钥推导公钥
    fn derive_ed25519_public_key(&self, private_key: &[u8]) -> ComponentResult<Vec<u8>> {
        if private_key.len() != 32 {
            return Err(ComponentError::CryptographyError(
                "Invalid private key length for Ed25519".to_string()
            ));
        }

        let signing_key = SigningKey::from_bytes(
            private_key.try_into()
                .map_err(|_| ComponentError::CryptographyError("Invalid private key format".to_string()))?
        );

        let verifying_key = signing_key.verifying_key();
        Ok(verifying_key.to_bytes().to_vec())
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

    /// 验证签名格式
    pub fn validate_signature_format(&self, signature: &[u8], algorithm: &str) -> ComponentResult<bool> {
        let alg = self.get_algorithm(algorithm)?;
        Ok(signature.len() == alg.signature_size())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_signature_engine_initialization() {
        let mut engine = SignatureEngine::new();
        assert!(!engine.is_initialized());

        engine.initialize().await.unwrap();
        assert!(engine.is_initialized());
    }

    #[tokio::test]
    async fn test_ecdsa_signature() {
        let mut engine = SignatureEngine::new();
        engine.initialize().await.unwrap();

        let data = b"Hello, Blockchain!";
        let (private_key, public_key) = engine.generate_keypair("ecdsa").unwrap();

        let signature = engine.sign(data, &private_key, "ecdsa").unwrap();
        assert!(engine.verify(data, &signature, &public_key, "ecdsa").unwrap());
    }

    #[tokio::test]
    async fn test_ed25519_signature() {
        let mut engine = SignatureEngine::new();
        engine.initialize().await.unwrap();

        let data = b"Hello, Blockchain!";
        let (private_key, public_key) = engine.generate_keypair("ed25519").unwrap();

        let signature = engine.sign(data, &private_key, "ed25519").unwrap();
        assert!(engine.verify(data, &signature, &public_key, "ed25519").unwrap());
    }

    #[tokio::test]
    async fn test_key_derivation() {
        let mut engine = SignatureEngine::new();
        engine.initialize().await.unwrap();

        let (private_key, expected_public_key) = engine.generate_keypair("ecdsa").unwrap();
        let derived_public_key = engine.derive_public_key(&private_key, "ecdsa").unwrap();

        assert_eq!(expected_public_key, derived_public_key);
    }

    #[tokio::test]
    async fn test_signature_verification_failure() {
        let mut engine = SignatureEngine::new();
        engine.initialize().await.unwrap();

        let data = b"Hello, Blockchain!";
        let (private_key, public_key) = engine.generate_keypair("ecdsa").unwrap();

        let signature = engine.sign(data, &private_key, "ecdsa").unwrap();
        let wrong_data = b"Wrong data";

        assert!(!engine.verify(wrong_data, &signature, &public_key, "ecdsa").unwrap());
    }

    #[tokio::test]
    async fn test_supported_algorithms() {
        let mut engine = SignatureEngine::new();
        engine.initialize().await.unwrap();

        let algorithms = engine.get_supported_algorithms();
        assert!(algorithms.contains(&"ecdsa".to_string()));
        assert!(algorithms.contains(&"ed25519".to_string()));
    }
}
