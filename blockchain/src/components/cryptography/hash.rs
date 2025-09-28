// 哈希引擎实现
use crate::components::{ComponentResult, ComponentError};
use sha2::{Sha256, Sha512, Digest};
// variable output blake2 variants used via fully qualified types
use std::collections::HashMap;

/// 哈希引擎
pub struct HashEngine {
    /// 哈希算法映射
    algorithms: HashMap<String, Box<dyn HashAlgorithm>>,
    /// 是否已初始化
    initialized: bool,
}

/// 哈希算法 trait
pub trait HashAlgorithm: Send + Sync {
    fn hash(&self, data: &[u8]) -> Vec<u8>;
    fn name(&self) -> &str;
}

/// SHA256 算法
pub struct Sha256Algorithm;

impl HashAlgorithm for Sha256Algorithm {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
    
    fn name(&self) -> &str {
        "sha256"
    }
}

/// SHA512 算法
pub struct Sha512Algorithm;

impl HashAlgorithm for Sha512Algorithm {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut hasher = Sha512::new();
        hasher.update(data);
        hasher.finalize().to_vec()
    }
    
    fn name(&self) -> &str {
        "sha512"
    }
}

/// Blake2b 算法
pub struct Blake2bAlgorithm;

impl HashAlgorithm for Blake2bAlgorithm {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use blake2::digest::{Update, VariableOutput};
        use blake2::Blake2bVar as Blake2bVar;
        let mut hasher = Blake2bVar::new(32).expect("valid size");
        hasher.update(data);
        let mut out = vec![0u8; 32];
        hasher.finalize_variable(&mut out).expect("valid output size");
        out
    }
    
    fn name(&self) -> &str {
        "blake2b"
    }
}

/// Blake2s 算法
pub struct Blake2sAlgorithm;

impl HashAlgorithm for Blake2sAlgorithm {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        use blake2::digest::{Update, VariableOutput};
        use blake2::Blake2sVar as Blake2sVar;
        let mut hasher = Blake2sVar::new(32).expect("valid size");
        hasher.update(data);
        let mut out = vec![0u8; 32];
        hasher.finalize_variable(&mut out).expect("valid output size");
        out
    }
    
    fn name(&self) -> &str {
        "blake2s"
    }
}

impl HashEngine {
    /// 创建新的哈希引擎
    pub fn new() -> Self {
        Self {
            algorithms: HashMap::new(),
            initialized: false,
        }
    }
    
    /// 初始化哈希引擎
    pub async fn initialize(&mut self) -> ComponentResult<()> {
        // 注册默认哈希算法
        self.register_algorithm("sha256", Box::new(Sha256Algorithm));
        self.register_algorithm("sha512", Box::new(Sha512Algorithm));
        self.register_algorithm("blake2b", Box::new(Blake2bAlgorithm));
        self.register_algorithm("blake2s", Box::new(Blake2sAlgorithm));
        
        self.initialized = true;
        Ok(())
    }
    
    /// 注册哈希算法
    pub fn register_algorithm(&mut self, name: &str, algorithm: Box<dyn HashAlgorithm>) {
        self.algorithms.insert(name.to_string(), algorithm);
    }
    
    /// 获取哈希算法
    fn get_algorithm(&self, name: &str) -> ComponentResult<&dyn HashAlgorithm> {
        self.algorithms.get(name)
            .map(|alg| alg.as_ref())
            .ok_or_else(|| ComponentError::CryptographyError(format!("Hash algorithm '{}' not found", name)))
    }
    
    /// SHA256 哈希
    pub fn sha256(&self, data: &[u8]) -> [u8; 32] {
        let hash = self.get_algorithm("sha256")
            .map(|alg| alg.hash(data))
            .unwrap_or_else(|_| {
                // 回退到直接实现
                let mut hasher = Sha256::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            });
        
        let mut result = [0u8; 32];
        result.copy_from_slice(&hash[..32]);
        result
    }
    
    /// SHA512 哈希
    pub fn sha512(&self, data: &[u8]) -> [u8; 64] {
        let hash = self.get_algorithm("sha512")
            .map(|alg| alg.hash(data))
            .unwrap_or_else(|_| {
                // 回退到直接实现
                let mut hasher = Sha512::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            });
        
        let mut result = [0u8; 64];
        result.copy_from_slice(&hash[..64]);
        result
    }
    
    /// Blake2b 哈希
    pub fn blake2b(&self, data: &[u8]) -> [u8; 64] {
        let hash = self.get_algorithm("blake2b")
            .map(|alg| alg.hash(data))
            .unwrap_or_else(|_| {
                // 回退到直接实现
                use blake2::digest::{Update, VariableOutput};
                use blake2::Blake2bVar as Blake2bVar;
                let mut hasher = Blake2bVar::new(32).expect("valid size");
                hasher.update(data);
                let mut out = vec![0u8; 32];
                hasher.finalize_variable(&mut out).expect("valid output size");
                out
            });
        
        let mut result = [0u8; 64];
        result.copy_from_slice(&hash[..64]);
        result
    }
    
    /// Blake2s 哈希
    pub fn blake2s(&self, data: &[u8]) -> [u8; 32] {
        let hash = self.get_algorithm("blake2s")
            .map(|alg| alg.hash(data))
            .unwrap_or_else(|_| {
                // 回退到直接实现
                use blake2::digest::{Update, VariableOutput};
                use blake2::Blake2sVar as Blake2sVar;
                let mut hasher = Blake2sVar::new(32).expect("valid size");
                hasher.update(data);
                let mut out = vec![0u8; 32];
                hasher.finalize_variable(&mut out).expect("valid output size");
                out
            });
        
        let mut result = [0u8; 32];
        result.copy_from_slice(&hash[..32]);
        result
    }
    
    /// 双 SHA256 哈希
    pub fn double_sha256(&self, data: &[u8]) -> [u8; 32] {
        let first_hash = self.sha256(data);
        self.sha256(&first_hash)
    }
    
    /// 哈希链
    pub fn hash_chain(&self, data: &[u8], iterations: usize) -> [u8; 32] {
        let mut result = self.sha256(data);
        
        for _ in 1..iterations {
            result = self.sha256(&result);
        }
        
        result
    }
    
    /// 计算 Merkle 根
    pub fn merkle_root(&self, hashes: &[[u8; 32]]) -> [u8; 32] {
        if hashes.is_empty() {
            return [0u8; 32];
        }
        
        if hashes.len() == 1 {
            return hashes[0];
        }
        
        let mut current_level = hashes.to_vec();
        
        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            
            for i in (0..current_level.len()).step_by(2) {
                let left = current_level[i];
                let right = if i + 1 < current_level.len() {
                    current_level[i + 1]
                } else {
                    left
                };
                
                let combined = self.hash_pair(left, right);
                next_level.push(combined);
            }
            
            current_level = next_level;
        }
        
        current_level[0]
    }
    
    /// 哈希两个值
    fn hash_pair(&self, left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
        let mut combined = Vec::new();
        combined.extend_from_slice(&left);
        combined.extend_from_slice(&right);
        self.sha256(&combined)
    }
    
    /// 验证哈希
    pub fn verify_hash(&self, data: &[u8], expected_hash: &[u8; 32]) -> bool {
        let actual_hash = self.sha256(data);
        actual_hash == *expected_hash
    }
    
    /// 获取支持的算法列表
    pub fn get_supported_algorithms(&self) -> Vec<String> {
        self.algorithms.keys().cloned().collect()
    }
    
    /// 检查是否已初始化
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_hash_engine_initialization() {
        let mut engine = HashEngine::new();
        assert!(!engine.is_initialized());
        
        engine.initialize().await.unwrap();
        assert!(engine.is_initialized());
    }
    
    #[tokio::test]
    async fn test_sha256_hashing() {
        let mut engine = HashEngine::new();
        engine.initialize().await.unwrap();
        
        let data = b"Hello, World!";
        let hash = engine.sha256(data);
        
        // 验证哈希长度
        assert_eq!(hash.len(), 32);
        
        // 验证相同输入产生相同输出
        let hash2 = engine.sha256(data);
        assert_eq!(hash, hash2);
    }
    
    #[tokio::test]
    async fn test_double_sha256() {
        let mut engine = HashEngine::new();
        engine.initialize().await.unwrap();
        
        let data = b"Test data";
        let single_hash = engine.sha256(data);
        let double_hash = engine.double_sha256(data);
        
        // 双哈希应该与单哈希不同
        assert_ne!(single_hash, double_hash);
    }
    
    #[tokio::test]
    async fn test_merkle_root() {
        let mut engine = HashEngine::new();
        engine.initialize().await.unwrap();
        
        let hashes = vec![
            [1u8; 32],
            [2u8; 32],
            [3u8; 32],
            [4u8; 32],
        ];
        
        let merkle_root = engine.merkle_root(&hashes);
        assert_eq!(merkle_root.len(), 32);
        
        // 测试奇数个哈希
        let odd_hashes = vec![
            [1u8; 32],
            [2u8; 32],
            [3u8; 32],
        ];
        
        let odd_merkle_root = engine.merkle_root(&odd_hashes);
        assert_eq!(odd_merkle_root.len(), 32);
    }
    
    #[tokio::test]
    async fn test_hash_verification() {
        let mut engine = HashEngine::new();
        engine.initialize().await.unwrap();
        
        let data = b"Test data";
        let hash = engine.sha256(data);
        
        assert!(engine.verify_hash(data, &hash));
        assert!(!engine.verify_hash(b"Different data", &hash));
    }
    
    #[tokio::test]
    async fn test_supported_algorithms() {
        let mut engine = HashEngine::new();
        engine.initialize().await.unwrap();
        
        let algorithms = engine.get_supported_algorithms();
        assert!(algorithms.contains(&"sha256".to_string()));
        assert!(algorithms.contains(&"sha512".to_string()));
        assert!(algorithms.contains(&"blake2b".to_string()));
        assert!(algorithms.contains(&"blake2s".to_string()));
    }
}
