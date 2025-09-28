// 区块结构定义
use serde::{Serialize, Deserialize};
use crate::core::{Transaction, Result, BlockchainError};
use std::time::{SystemTime, UNIX_EPOCH};

/// 区块结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// 区块头
    pub header: BlockHeader,
    
    /// 交易列表
    pub transactions: Vec<Transaction>,
    
    /// Merkle根
    pub merkle_root: [u8; 32],
    
    /// 区块哈希
    pub block_hash: [u8; 32],
}

/// 区块头结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    /// 版本号
    pub version: u32,
    
    /// 前一个区块的哈希
    pub previous_hash: [u8; 32],
    
    /// Merkle根
    pub merkle_root: [u8; 32],
    
    /// 时间戳
    pub timestamp: u64,
    
    /// 难度值
    pub difficulty: u32,
    
    /// 随机数
    pub nonce: u64,
    
    /// 区块高度
    pub height: u64,
    
    /// 区块哈希
    pub block_hash: [u8; 32],
}

impl Block {
    /// 创建新区块
    pub fn new(
        previous_hash: [u8; 32],
        transactions: Vec<Transaction>,
        height: u64,
        difficulty: u32,
    ) -> Result<Self> {
        // 计算Merkle根
        let merkle_root = Self::calculate_merkle_root(&transactions);
        
        // 创建区块头
        let header = BlockHeader {
            version: 1,
            previous_hash,
            merkle_root,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            difficulty,
            nonce: 0,
            height,
            block_hash: [0u8; 32], // 将在挖矿时计算
        };
        
        // 计算区块哈希
        let block_hash = Self::calculate_block_hash(&header);
        
        Ok(Block {
            header: BlockHeader {
                block_hash,
                ..header
            },
            transactions,
            merkle_root,
            block_hash,
        })
    }
    
    /// 创建创世区块
    pub fn create_genesis_block() -> Result<Self> {
        let transactions = vec![]; // 创世区块通常没有交易
        let previous_hash = [0u8; 32]; // 创世区块的前一个哈希为0
        
        Self::new(previous_hash, transactions, 0, 1)
    }
    
    /// 计算Merkle根
    fn calculate_merkle_root(transactions: &[Transaction]) -> [u8; 32] {
        if transactions.is_empty() {
            return [0u8; 32];
        }
        
        let mut hashes: Vec<[u8; 32]> = transactions
            .iter()
            .map(|tx| tx.hash())
            .collect();
        
        // 如果交易数量为奇数，复制最后一个哈希
        if hashes.len() % 2 == 1 {
            hashes.push(*hashes.last().unwrap());
        }
        
        // 计算Merkle根
        while hashes.len() > 1 {
            let mut next_level = Vec::new();
            
            for i in (0..hashes.len()).step_by(2) {
                let left = hashes[i];
                let right = hashes[i + 1];
                let combined = Self::hash_pair(left, right);
                next_level.push(combined);
            }
            
            // 如果下一层哈希数量为奇数，复制最后一个
            if next_level.len() % 2 == 1 && next_level.len() > 1 {
                next_level.push(*next_level.last().unwrap());
            }
            
            hashes = next_level;
        }
        
        hashes[0]
    }
    
    /// 计算区块哈希
    fn calculate_block_hash(header: &BlockHeader) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(&header.version.to_be_bytes());
        hasher.update(&header.previous_hash);
        hasher.update(&header.merkle_root);
        hasher.update(&header.timestamp.to_be_bytes());
        hasher.update(&header.difficulty.to_be_bytes());
        hasher.update(&header.nonce.to_be_bytes());
        hasher.update(&header.height.to_be_bytes());
        
        hasher.finalize().into()
    }
    
    /// 哈希两个值
    fn hash_pair(left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(&left);
        hasher.update(&right);
        hasher.finalize().into()
    }
    
    /// 验证区块
    pub fn validate(&self) -> Result<()> {
        // 1. 验证区块头
        self.header.validate()?;
        
        // 2. 验证Merkle根
        let calculated_merkle_root = Self::calculate_merkle_root(&self.transactions);
        if self.merkle_root != calculated_merkle_root {
            return Err(BlockchainError::InvalidBlock("Invalid merkle root".to_string()));
        }
        
        // 3. 验证区块哈希
        let calculated_hash = Self::calculate_block_hash(&self.header);
        if self.block_hash != calculated_hash {
            return Err(BlockchainError::InvalidBlock("Invalid block hash".to_string()));
        }
        
        // 4. 验证交易
        for tx in &self.transactions {
            tx.validate()?;
        }
        
        Ok(())
    }
    
    /// 序列化区块
    pub fn serialize(&self) -> Result<Vec<u8>> {
        bincode::serialize(self)
            .map_err(|e| BlockchainError::InvalidBlock(format!("Serialization failed: {}", e)))
    }
    
    /// 反序列化区块
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        bincode::deserialize(data)
            .map_err(|e| BlockchainError::InvalidBlock(format!("Deserialization failed: {}", e)))
    }
    
    /// 获取区块大小
    pub fn size(&self) -> usize {
        self.serialize().unwrap_or_default().len()
    }
    
    /// 获取交易数量
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }
    
    /// 获取区块高度
    pub fn height(&self) -> u64 {
        self.header.height
    }
    
    /// 获取时间戳
    pub fn timestamp(&self) -> u64 {
        self.header.timestamp
    }
    
    /// 获取难度
    pub fn difficulty(&self) -> u32 {
        self.header.difficulty
    }
    
    /// 获取随机数
    pub fn nonce(&self) -> u64 {
        self.header.nonce
    }
    
    /// 设置随机数（用于挖矿）
    pub fn set_nonce(&mut self, nonce: u64) {
        self.header.nonce = nonce;
        self.header.block_hash = Self::calculate_block_hash(&self.header);
        self.block_hash = self.header.block_hash;
    }
}

impl BlockHeader {
    /// 验证区块头
    pub fn validate(&self) -> Result<()> {
        // 1. 验证版本号
        if self.version == 0 {
            return Err(BlockchainError::InvalidBlock("Invalid version".to_string()));
        }
        
        // 2. 验证时间戳
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // 时间戳不能太早或太晚（允许2小时的误差）
        if self.timestamp > current_time + 7200 || self.timestamp < current_time - 7200 {
            return Err(BlockchainError::InvalidBlock("Invalid timestamp".to_string()));
        }
        
        // 3. 验证难度
        if self.difficulty == 0 {
            return Err(BlockchainError::InvalidBlock("Invalid difficulty".to_string()));
        }
        
        // 4. 验证高度
        if self.height == 0 && self.previous_hash != [0u8; 32] {
            return Err(BlockchainError::InvalidBlock("Invalid genesis block".to_string()));
        }
        
        Ok(())
    }
    
    /// 序列化区块头
    pub fn serialize(&self) -> Result<Vec<u8>> {
        bincode::serialize(self)
            .map_err(|e| BlockchainError::InvalidBlock(format!("Header serialization failed: {}", e)))
    }
    
    /// 反序列化区块头
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        bincode::deserialize(data)
            .map_err(|e| BlockchainError::InvalidBlock(format!("Header deserialization failed: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_block_creation() {
        let transactions = vec![];
        let previous_hash = [1u8; 32];
        let block = Block::new(previous_hash, transactions, 1, 1).unwrap();
        
        assert_eq!(block.height(), 1);
        assert_eq!(block.header.previous_hash, previous_hash);
    }
    
    #[test]
    fn test_genesis_block() {
        let genesis = Block::create_genesis_block().unwrap();
        
        assert_eq!(genesis.height(), 0);
        assert_eq!(genesis.header.previous_hash, [0u8; 32]);
        assert!(genesis.transactions.is_empty());
    }
    
    #[test]
    fn test_block_validation() {
        let block = Block::create_genesis_block().unwrap();
        assert!(block.validate().is_ok());
    }
    
    #[test]
    fn test_merkle_root_calculation() {
        let transactions = vec![];
        let merkle_root = Block::calculate_merkle_root(&transactions);
        assert_eq!(merkle_root, [0u8; 32]);
    }
}
