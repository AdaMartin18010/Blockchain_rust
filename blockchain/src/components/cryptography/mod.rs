// 密码学组件模块
pub mod hash;
pub mod signature;
pub mod encryption;

pub use hash::HashEngine;
pub use signature::SignatureEngine;
pub use encryption::EncryptionEngine;

//use crate::core::{Result, BlockchainError};
use crate::components::{ComponentResult};

/// 密码学组件
pub struct CryptographyComponent {
    pub hash_engine: HashEngine,
    pub signature_engine: SignatureEngine,
    pub encryption_engine: EncryptionEngine,
}

impl CryptographyComponent {
    /// 创建新的密码学组件
    pub fn new() -> Self {
        Self {
            hash_engine: HashEngine::new(),
            signature_engine: SignatureEngine::new(),
            encryption_engine: EncryptionEngine::new(),
        }
    }
    
    /// 初始化密码学组件
    pub async fn initialize(&mut self) -> ComponentResult<()> {
        self.hash_engine.initialize().await?;
        self.signature_engine.initialize().await?;
        self.encryption_engine.initialize().await?;
        Ok(())
    }
    
    /// 哈希数据
    pub fn hash_data(&self, data: &[u8]) -> [u8; 32] {
        self.hash_engine.sha256(data)
    }
    
    /// 签名交易
    pub fn sign_transaction(&self, tx: &[u8], private_key: &[u8]) -> ComponentResult<Vec<u8>> {
        self.signature_engine.sign(tx, private_key, "ecdsa")
    }
    
    /// 验证签名
    pub fn verify_signature(&self, tx: &[u8], signature: &[u8], public_key: &[u8]) -> ComponentResult<bool> {
        self.signature_engine.verify(tx, signature, public_key, "ecdsa")
    }
    
    /// 加密数据
    pub fn encrypt_data(&self, data: &[u8], key: &[u8]) -> ComponentResult<Vec<u8>> {
        self.encryption_engine.encrypt(data, key, "aes-gcm")
    }
    
    /// 解密数据
    pub fn decrypt_data(&self, encrypted_data: &[u8], key: &[u8]) -> ComponentResult<Vec<u8>> {
        self.encryption_engine.decrypt(encrypted_data, key, "aes-gcm")
    }
}
