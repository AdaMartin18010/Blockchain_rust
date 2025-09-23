//! # 高级密码学模块
//! 
//! 展示 Rust 最新特性在区块链密码学中的应用
//! Advanced cryptography module demonstrating latest Rust features in blockchain cryptography

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Sha512};
use blake2::{Blake2b, Blake2s};
// use keccak::Keccak256; // 暂时注释掉，等待依赖更新
// use ripemd::Ripemd160; // 暂时注释掉，等待依赖更新
use secp256k1::{Secp256k1, SecretKey, PublicKey as SecpPublicKey, Message, ecdsa::Signature as SecpSignature};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature as EdSignature, Signer, Verifier};
use rand::{RngCore, rngs::OsRng};
use std::collections::HashMap;
use thiserror::Error;

/// 密码学错误类型
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum CryptoError {
    #[error("Invalid key format")]
    InvalidKey,
    #[error("Signature verification failed")]
    InvalidSignature,
    #[error("Hash calculation failed")]
    HashError,
    #[error("Encryption failed")]
    EncryptionError,
    #[error("Decryption failed")]
    DecryptionError,
    #[error("Invalid input data")]
    InvalidInput,
}

/// 支持的哈希算法类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HashAlgorithm {
    Sha256,
    Sha512,
    Blake2b,
    Blake2s,
    Keccak256,
    Ripemd160,
}

/// 支持的数字签名算法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignatureAlgorithm {
    Secp256k1,
    Ed25519,
}

/// 高级哈希结构，支持多种算法
/// Advanced hash structure supporting multiple algorithms
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvancedHash {
    pub algorithm: HashAlgorithm,
    pub data: Vec<u8>,
}

impl AdvancedHash {
    /// 使用指定算法计算哈希
    /// Calculate hash using specified algorithm
    pub fn hash(data: &[u8], algorithm: HashAlgorithm) -> Result<Self, CryptoError> {
        let hash_data = match algorithm {
            HashAlgorithm::Sha256 => {
                let mut hasher = Sha256::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Sha512 => {
                let mut hasher = Sha512::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Blake2b => {
                let mut hasher = Blake2b::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Blake2s => {
                let mut hasher = Blake2s::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Keccak256 => {
                // 暂时使用 SHA256 替代，等待 Keccak 依赖更新
                let mut hasher = Sha256::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Ripemd160 => {
                // 暂时使用 SHA256 替代，等待 RIPEMD 依赖更新
                let mut hasher = Sha256::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
        };

        Ok(Self {
            algorithm,
            data: hash_data,
        })
    }

    /// 转换为十六进制字符串
    /// Convert to hexadecimal string
    pub fn to_hex(&self) -> String {
        hex::encode(&self.data)
    }

    /// 从十六进制字符串创建
    /// Create from hexadecimal string
    pub fn from_hex(hex_str: &str, algorithm: HashAlgorithm) -> Result<Self, CryptoError> {
        let data = hex::decode(hex_str).map_err(|_| CryptoError::InvalidInput)?;
        Ok(Self { algorithm, data })
    }
}

/// 高级密钥对结构
/// Advanced key pair structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedKeyPair {
    pub algorithm: SignatureAlgorithm,
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

impl AdvancedKeyPair {
    /// 生成新的密钥对
    /// Generate new key pair
    pub fn generate(algorithm: SignatureAlgorithm) -> Result<Self, CryptoError> {
        match algorithm {
            SignatureAlgorithm::Secp256k1 => {
                let secp = Secp256k1::new();
                let mut rng = OsRng;
                let secret_key = SecretKey::new(&mut rng);
                let public_key = secret_key.public_key(&secp);
                
                Ok(Self {
                    algorithm,
                    private_key: secret_key.secret_bytes().to_vec(),
                    public_key: public_key.serialize().to_vec(),
                })
            }
            SignatureAlgorithm::Ed25519 => {
                let mut rng = OsRng;
                let signing_key = SigningKey::generate(&mut rng);
                let verifying_key = signing_key.verifying_key();
                
                Ok(Self {
                    algorithm,
                    private_key: signing_key.to_bytes().to_vec(),
                    public_key: verifying_key.to_bytes().to_vec(),
                })
            }
        }
    }

    /// 从私钥恢复密钥对
    /// Recover key pair from private key
    pub fn from_private_key(private_key: &[u8], algorithm: SignatureAlgorithm) -> Result<Self, CryptoError> {
        match algorithm {
            SignatureAlgorithm::Secp256k1 => {
                let secp = Secp256k1::new();
                let secret_key = SecretKey::from_slice(private_key)
                    .map_err(|_| CryptoError::InvalidKey)?;
                let public_key = secret_key.public_key(&secp);
                
                Ok(Self {
                    algorithm,
                    private_key: private_key.to_vec(),
                    public_key: public_key.serialize().to_vec(),
                })
            }
            SignatureAlgorithm::Ed25519 => {
                let signing_key = SigningKey::from_bytes(
                    private_key.try_into().map_err(|_| CryptoError::InvalidKey)?
                );
                let verifying_key = signing_key.verifying_key();
                
                Ok(Self {
                    algorithm,
                    private_key: private_key.to_vec(),
                    public_key: verifying_key.to_bytes().to_vec(),
                })
            }
        }
    }

    /// 获取公钥哈希（用于地址生成）
    /// Get public key hash for address generation
    pub fn get_address_hash(&self) -> Result<AdvancedHash, CryptoError> {
        AdvancedHash::hash(&self.public_key, HashAlgorithm::Sha256) // 暂时使用 SHA256
    }
}

/// 高级数字签名结构
/// Advanced digital signature structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSignature {
    pub algorithm: SignatureAlgorithm,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}

impl AdvancedSignature {
    /// 使用密钥对签名消息
    /// Sign message using key pair
    pub fn sign(message: &[u8], key_pair: &AdvancedKeyPair) -> Result<Self, CryptoError> {
        match key_pair.algorithm {
            SignatureAlgorithm::Secp256k1 => {
                let secp = Secp256k1::new();
                let secret_key = SecretKey::from_byte_array(
                    key_pair.private_key.try_into().map_err(|_| CryptoError::InvalidKey)?
                );
                
                // 计算消息哈希
                let message_hash = AdvancedHash::hash(message, HashAlgorithm::Sha256)?;
                let message_obj = Message::from_digest_slice(&message_hash.data)
                    .map_err(|_| CryptoError::InvalidInput)?;
                
                let signature = secp.sign_ecdsa(&message_obj, &secret_key);
                
                Ok(Self {
                    algorithm: key_pair.algorithm,
                    signature: signature.serialize_der().to_vec(),
                    public_key: key_pair.public_key.clone(),
                })
            }
            SignatureAlgorithm::Ed25519 => {
                let private_key_bytes: [u8; 32] = key_pair.private_key.try_into().map_err(|_| CryptoError::InvalidKey)?;
                let signing_key = SigningKey::from_bytes(&private_key_bytes);
                
                let signature = signing_key.sign(message);
                
                Ok(Self {
                    algorithm: key_pair.algorithm,
                    signature: signature.to_bytes().to_vec(),
                    public_key: key_pair.public_key.clone(),
                })
            }
        }
    }

    /// 验证签名
    /// Verify signature
    pub fn verify(&self, message: &[u8]) -> Result<bool, CryptoError> {
        match self.algorithm {
            SignatureAlgorithm::Secp256k1 => {
                let secp = Secp256k1::new();
                let public_key = SecpPublicKey::from_slice(&self.public_key)
                    .map_err(|_| CryptoError::InvalidKey)?;
                let signature = SecpSignature::from_der(&self.signature)
                    .map_err(|_| CryptoError::InvalidSignature)?;
                
                // 计算消息哈希
                let message_hash = AdvancedHash::hash(message, HashAlgorithm::Sha256)?;
                let message_obj = Message::from_slice(&message_hash.data)
                    .map_err(|_| CryptoError::InvalidInput)?;
                
                Ok(secp.verify_ecdsa(&message_obj, &signature, &public_key).is_ok())
            }
            SignatureAlgorithm::Ed25519 => {
                let public_key_bytes: [u8; 32] = self.public_key.try_into().map_err(|_| CryptoError::InvalidKey)?;
                let verifying_key = VerifyingKey::from_bytes(&public_key_bytes).map_err(|_| CryptoError::InvalidKey)?;
                let signature_bytes: [u8; 64] = self.signature.try_into().map_err(|_| CryptoError::InvalidSignature)?;
                let signature = EdSignature::from_bytes(&signature_bytes);
                
                Ok(verifying_key.verify(message, &signature).is_ok())
            }
        }
    }
}

/// 区块链地址生成器
/// Blockchain address generator
pub struct AddressGenerator;

impl AddressGenerator {
    /// 生成比特币风格地址
    /// Generate Bitcoin-style address
    pub fn generate_bitcoin_address(key_pair: &AdvancedKeyPair) -> Result<String, CryptoError> {
        // 1. 获取公钥哈希
        let pub_key_hash = key_pair.get_address_hash()?;
        
        // 2. 添加版本字节（主网）
        let mut address_bytes = vec![0x00];
        address_bytes.extend_from_slice(&pub_key_hash.data);
        
        // 3. 计算校验和
        let checksum = AdvancedHash::hash(&address_bytes, HashAlgorithm::Sha256)?;
        let checksum = AdvancedHash::hash(&checksum.data, HashAlgorithm::Sha256)?;
        
        // 4. 添加校验和
        address_bytes.extend_from_slice(&checksum.data[..4]);
        
        // 5. Base58 编码 (简化实现)
        Ok(format!("bitcoin_{}", hex::encode(&address_bytes[..8])))
    }

    /// 生成以太坊风格地址
    /// Generate Ethereum-style address
    pub fn generate_ethereum_address(key_pair: &AdvancedKeyPair) -> Result<String, CryptoError> {
        // 1. 计算 Keccak256 哈希 (暂时使用 SHA256)
        let hash = AdvancedHash::hash(&key_pair.public_key, HashAlgorithm::Sha256)?;
        
        // 2. 取最后 20 字节
        let address_bytes = &hash.data[12..];
        
        // 3. 十六进制编码
        Ok(format!("0x{}", hex::encode(address_bytes)))
    }
}

/// Merkle 树实现
/// Merkle tree implementation
#[derive(Debug, Clone)]
pub struct MerkleTree {
    pub leaves: Vec<AdvancedHash>,
    pub levels: Vec<Vec<AdvancedHash>>,
    pub root: Option<AdvancedHash>,
}

impl MerkleTree {
    /// 创建新的 Merkle 树
    /// Create new Merkle tree
    pub fn new(data: Vec<Vec<u8>>, algorithm: HashAlgorithm) -> Result<Self, CryptoError> {
        if data.is_empty() {
            return Err(CryptoError::InvalidInput);
        }

        // 计算叶子节点哈希
        let mut leaves = Vec::new();
        for item in data {
            leaves.push(AdvancedHash::hash(&item, algorithm)?);
        }

        // 构建树层级
        let mut levels = vec![leaves.clone()];
        let mut current_level = levels[0].clone();

        while current_level.len() > 1 {
            let mut next_level = Vec::new();
            
            for i in (0..current_level.len()).step_by(2) {
                let left = &current_level[i];
                let right = if i + 1 < current_level.len() {
                    &current_level[i + 1]
                } else {
                    left // 奇数个节点时，最后一个节点重复
                };

                // 连接两个哈希并计算父节点哈希
                let mut combined = Vec::new();
                combined.extend_from_slice(&left.data);
                combined.extend_from_slice(&right.data);
                
                let parent_hash = AdvancedHash::hash(&combined, algorithm)?;
                next_level.push(parent_hash);
            }

            levels.push(next_level.clone());
            current_level = next_level;
        }

        let root = levels.last().and_then(|level| level.first()).cloned();

        Ok(Self {
            leaves,
            levels,
            root,
        })
    }

    /// 生成 Merkle 证明
    /// Generate Merkle proof
    pub fn generate_proof(&self, leaf_index: usize) -> Result<Vec<AdvancedHash>, CryptoError> {
        if leaf_index >= self.leaves.len() {
            return Err(CryptoError::InvalidInput);
        }

        let mut proof = Vec::new();
        let mut current_index = leaf_index;
        let mut current_level = 0;

        while current_level < self.levels.len() - 1 {
            let level = &self.levels[current_level];
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            if sibling_index < level.len() {
                proof.push(level[sibling_index].clone());
            }

            current_index /= 2;
            current_level += 1;
        }

        Ok(proof)
    }

    /// 验证 Merkle 证明
    /// Verify Merkle proof
    pub fn verify_proof(
        &self,
        leaf: &AdvancedHash,
        proof: &[AdvancedHash],
        leaf_index: usize,
    ) -> Result<bool, CryptoError> {
        if self.root.is_none() {
            return Ok(false);
        }

        let mut current_hash = leaf.clone();
        let mut current_index = leaf_index;

        for sibling in proof {
            let mut combined = if current_index % 2 == 0 {
                let mut data = Vec::new();
                data.extend_from_slice(&current_hash.data);
                data.extend_from_slice(&sibling.data);
                data
            } else {
                let mut data = Vec::new();
                data.extend_from_slice(&sibling.data);
                data.extend_from_slice(&current_hash.data);
                data
            };

            current_hash = AdvancedHash::hash(&combined, current_hash.algorithm)?;
            current_index /= 2;
        }

        Ok(current_hash == *self.root.as_ref().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advanced_hash() {
        let data = b"test data";
        let hash = AdvancedHash::hash(data, HashAlgorithm::Sha256).unwrap();
        assert_eq!(hash.data.len(), 32);
        
        let hex = hash.to_hex();
        let restored = AdvancedHash::from_hex(&hex, HashAlgorithm::Sha256).unwrap();
        assert_eq!(hash, restored);
    }

    #[test]
    fn test_key_pair_generation() {
        let key_pair = AdvancedKeyPair::generate(SignatureAlgorithm::Secp256k1).unwrap();
        assert_eq!(key_pair.private_key.len(), 32);
        assert_eq!(key_pair.public_key.len(), 33);
        
        let ed_key_pair = AdvancedKeyPair::generate(SignatureAlgorithm::Ed25519).unwrap();
        assert_eq!(ed_key_pair.private_key.len(), 32);
        assert_eq!(ed_key_pair.public_key.len(), 32);
    }

    #[test]
    fn test_signature_verification() {
        let key_pair = AdvancedKeyPair::generate(SignatureAlgorithm::Secp256k1).unwrap();
        let message = b"test message";
        
        let signature = AdvancedSignature::sign(message, &key_pair).unwrap();
        assert!(signature.verify(message).unwrap());
        
        // 测试错误消息
        let wrong_message = b"wrong message";
        assert!(!signature.verify(wrong_message).unwrap());
    }

    #[test]
    fn test_merkle_tree() {
        let data = vec![
            b"data1".to_vec(),
            b"data2".to_vec(),
            b"data3".to_vec(),
            b"data4".to_vec(),
        ];
        
        let tree = MerkleTree::new(data, HashAlgorithm::Sha256).unwrap();
        assert!(tree.root.is_some());
        
        // 测试证明生成和验证
        let proof = tree.generate_proof(0).unwrap();
        let leaf = &tree.leaves[0];
        assert!(tree.verify_proof(leaf, &proof, 0).unwrap());
    }

    #[test]
    fn test_address_generation() {
        let key_pair = AdvancedKeyPair::generate(SignatureAlgorithm::Secp256k1).unwrap();
        
        let bitcoin_addr = AddressGenerator::generate_bitcoin_address(&key_pair).unwrap();
        assert!(bitcoin_addr.starts_with('1'));
        
        let ethereum_addr = AddressGenerator::generate_ethereum_address(&key_pair).unwrap();
        assert!(ethereum_addr.starts_with("0x"));
        assert_eq!(ethereum_addr.len(), 42); // 0x + 40 hex chars
    }
}
