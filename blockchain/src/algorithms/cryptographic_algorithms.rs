//! 密码学算法实现

use super::{AlgorithmResult, AlgorithmError};

/// 密码学算法
#[derive(Debug)]
pub struct CryptographicAlgorithms {
    // 密码学算法相关状态
}

impl CryptographicAlgorithms {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_random_bytes(&self, length: usize) -> Vec<u8> {
        use rand::RngCore;
        let mut rng = rand::rng();
        let mut bytes = vec![0u8; length];
        rng.fill_bytes(&mut bytes);
        bytes
    }

    pub fn calculate_merkle_root(&self, hashes: &[Vec<u8>]) -> AlgorithmResult<Vec<u8>> {
        if hashes.is_empty() {
            return Err(AlgorithmError::InvalidParameters("哈希列表不能为空".to_string()).into());
        }

        if hashes.len() == 1 {
            return Ok(hashes[0].clone());
        }

        // 简化的Merkle根计算
        let mut current_level = hashes.to_vec();
        
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            
            for i in (0..current_level.len()).step_by(2) {
                let left = &current_level[i];
                let right = if i + 1 < current_level.len() {
                    &current_level[i + 1]
                } else {
                    &current_level[i]
                };
                
                let combined = [left.as_slice(), right.as_slice()].concat();
                let hash = self.hash(&combined);
                next_level.push(hash);
            }
            
            current_level = next_level;
        }

        Ok(current_level[0].clone())
    }

    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
}
