//! # 数据库持久化模块
//! 
//! 提供区块链数据的持久化存储功能

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

/// 数据库错误类型
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseError {
    #[error("Database connection failed")]
    ConnectionFailed,
    #[error("Serialization failed")]
    SerializationFailed,
    #[error("Deserialization failed")]
    DeserializationFailed,
    #[error("Key not found")]
    KeyNotFound,
    #[error("IO error: {0}")]
    IoError(String),
}

/// 数据库接口 trait
pub trait Database {
    /// 存储键值对
    fn put(&mut self, key: &str, value: &[u8]) -> Result<(), DatabaseError>;
    
    /// 获取值
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>, DatabaseError>;
    
    /// 删除键
    fn delete(&mut self, key: &str) -> Result<(), DatabaseError>;
    
    /// 检查键是否存在
    fn exists(&self, key: &str) -> Result<bool, DatabaseError>;
    
    /// 获取所有键
    fn keys(&self) -> Result<Vec<String>, DatabaseError>;
    
    /// 清空数据库
    fn clear(&mut self) -> Result<(), DatabaseError>;
}

/// 内存数据库实现
#[derive(Debug, Clone)]
pub struct MemoryDatabase {
    data: HashMap<String, Vec<u8>>,
}

impl MemoryDatabase {
    /// 创建新的内存数据库
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl Default for MemoryDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl Database for MemoryDatabase {
    fn put(&mut self, key: &str, value: &[u8]) -> Result<(), DatabaseError> {
        self.data.insert(key.to_string(), value.to_vec());
        Ok(())
    }

    fn get(&self, key: &str) -> Result<Option<Vec<u8>>, DatabaseError> {
        Ok(self.data.get(key).cloned())
    }

    fn delete(&mut self, key: &str) -> Result<(), DatabaseError> {
        self.data.remove(key);
        Ok(())
    }

    fn exists(&self, key: &str) -> Result<bool, DatabaseError> {
        Ok(self.data.contains_key(key))
    }

    fn keys(&self) -> Result<Vec<String>, DatabaseError> {
        Ok(self.data.keys().cloned().collect())
    }

    fn clear(&mut self) -> Result<(), DatabaseError> {
        self.data.clear();
        Ok(())
    }
}

/// 文件数据库实现
#[derive(Debug)]
pub struct FileDatabase {
    path: String,
    data: HashMap<String, Vec<u8>>,
    dirty: bool,
}

impl FileDatabase {
    /// 创建新的文件数据库
    pub fn new(path: &str) -> Result<Self, DatabaseError> {
        let mut db = Self {
            path: path.to_string(),
            data: HashMap::new(),
            dirty: false,
        };
        
        // 尝试加载现有数据
        if Path::new(path).exists() {
            db.load()?;
        }
        
        Ok(db)
    }
    
    /// 从文件加载数据
    fn load(&mut self) -> Result<(), DatabaseError> {
        let content = std::fs::read_to_string(&self.path)
            .map_err(|e| DatabaseError::IoError(e.to_string()))?;
        
        if !content.is_empty() {
            let loaded_data: HashMap<String, Vec<u8>> = serde_json::from_str(&content)
                .map_err(|_| DatabaseError::DeserializationFailed)?;
            self.data = loaded_data;
        }
        
        Ok(())
    }
    
    /// 保存数据到文件
    fn save(&self) -> Result<(), DatabaseError> {
        let content = serde_json::to_string(&self.data)
            .map_err(|_| DatabaseError::SerializationFailed)?;
        
        std::fs::write(&self.path, content)
            .map_err(|e| DatabaseError::IoError(e.to_string()))?;
        
        Ok(())
    }
}

impl Drop for FileDatabase {
    fn drop(&mut self) {
        if self.dirty {
            let _ = self.save();
        }
    }
}

impl Database for FileDatabase {
    fn put(&mut self, key: &str, value: &[u8]) -> Result<(), DatabaseError> {
        self.data.insert(key.to_string(), value.to_vec());
        self.dirty = true;
        self.save()?;
        Ok(())
    }

    fn get(&self, key: &str) -> Result<Option<Vec<u8>>, DatabaseError> {
        Ok(self.data.get(key).cloned())
    }

    fn delete(&mut self, key: &str) -> Result<(), DatabaseError> {
        if self.data.remove(key).is_some() {
            self.dirty = true;
            self.save()?;
        }
        Ok(())
    }

    fn exists(&self, key: &str) -> Result<bool, DatabaseError> {
        Ok(self.data.contains_key(key))
    }

    fn keys(&self) -> Result<Vec<String>, DatabaseError> {
        Ok(self.data.keys().cloned().collect())
    }

    fn clear(&mut self) -> Result<(), DatabaseError> {
        self.data.clear();
        self.dirty = true;
        self.save()?;
        Ok(())
    }
}

/// 区块链存储管理器
pub struct BlockchainStorage<D: Database> {
    db: D,
}

impl<D: Database> BlockchainStorage<D> {
    /// 创建新的区块链存储管理器
    pub fn new(db: D) -> Self {
        Self { db }
    }
    
    /// 保存区块
    pub fn save_block(&mut self, index: u64, block: &[u8]) -> Result<(), DatabaseError> {
        let key = format!("block_{}", index);
        self.db.put(&key, block)?;
        Ok(())
    }
    
    /// 加载区块
    pub fn load_block(&self, index: u64) -> Result<Option<Vec<u8>>, DatabaseError> {
        let key = format!("block_{}", index);
        self.db.get(&key)
    }
    
    /// 保存交易
    pub fn save_transaction(&mut self, tx_id: &str, transaction: &[u8]) -> Result<(), DatabaseError> {
        let key = format!("tx_{}", tx_id);
        self.db.put(&key, transaction)?;
        Ok(())
    }
    
    /// 加载交易
    pub fn load_transaction(&self, tx_id: &str) -> Result<Option<Vec<u8>>, DatabaseError> {
        let key = format!("tx_{}", tx_id);
        self.db.get(&key)
    }
    
    /// 保存区块链状态
    pub fn save_state(&mut self, state: &[u8]) -> Result<(), DatabaseError> {
        self.db.put("blockchain_state", state)?;
        Ok(())
    }
    
    /// 加载区块链状态
    pub fn load_state(&self) -> Result<Option<Vec<u8>>, DatabaseError> {
        self.db.get("blockchain_state")
    }
    
    /// 获取所有区块索引
    pub fn get_block_indices(&self) -> Result<Vec<u64>, DatabaseError> {
        let keys = self.db.keys()?;
        let mut indices = Vec::new();
        
        for key in keys {
            if key.starts_with("block_") {
                if let Ok(index_str) = key.strip_prefix("block_") {
                    if let Ok(index) = index_str.parse::<u64>() {
                        indices.push(index);
                    }
                }
            }
        }
        
        indices.sort();
        Ok(indices)
    }
    
    /// 获取所有交易ID
    pub fn get_transaction_ids(&self) -> Result<Vec<String>, DatabaseError> {
        let keys = self.db.keys()?;
        let mut tx_ids = Vec::new();
        
        for key in keys {
            if key.starts_with("tx_") {
                if let Some(tx_id) = key.strip_prefix("tx_") {
                    tx_ids.push(tx_id.to_string());
                }
            }
        }
        
        Ok(tx_ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_database() {
        let mut db = MemoryDatabase::new();
        
        // 测试基本操作
        db.put("key1", b"value1").unwrap();
        assert!(db.exists("key1").unwrap());
        assert_eq!(db.get("key1").unwrap(), Some(b"value1".to_vec()));
        
        // 测试删除
        db.delete("key1").unwrap();
        assert!(!db.exists("key1").unwrap());
        
        // 测试清空
        db.put("key2", b"value2").unwrap();
        db.clear().unwrap();
        assert!(db.keys().unwrap().is_empty());
    }

    #[test]
    fn test_blockchain_storage() {
        let db = MemoryDatabase::new();
        let mut storage = BlockchainStorage::new(db);
        
        // 测试区块存储
        let block_data = b"test block data";
        storage.save_block(0, block_data).unwrap();
        
        let loaded_block = storage.load_block(0).unwrap();
        assert_eq!(loaded_block, Some(block_data.to_vec()));
        
        // 测试交易存储
        let tx_data = b"test transaction data";
        storage.save_transaction("tx123", tx_data).unwrap();
        
        let loaded_tx = storage.load_transaction("tx123").unwrap();
        assert_eq!(loaded_tx, Some(tx_data.to_vec()));
    }
}
