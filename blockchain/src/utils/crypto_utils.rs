//! 密码学工具实现

// use super::{UtilsResult, UtilsError};
// use crate::core::{Result, BlockchainError};

/// 密码学工具
#[derive(Debug)]
pub struct CryptoUtils {
    // 密码学工具相关状态
}

impl CryptoUtils {
    pub fn new() -> Self {
        Self {}
    }

    pub fn hash(&self, data: &[u8]) -> Vec<u8> {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }

    pub fn generate_random_bytes(&self, length: usize) -> Vec<u8> {
        use rand::RngCore;
        let mut rng = rand::rng();
        let mut bytes = vec![0u8; length];
        rng.fill_bytes(&mut bytes);
        bytes
    }
}
