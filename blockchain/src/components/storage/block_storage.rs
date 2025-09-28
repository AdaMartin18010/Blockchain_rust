//! 区块存储实现

use super::{StorageComponent, StorageResult, StorageStats};
use crate::core::{Block};

/// 区块存储实现
#[derive(Debug)]
pub struct BlockStorage {
    blocks: std::collections::HashMap<u64, Block>,
    block_hashes: std::collections::HashMap<[u8; 32], u64>,
}

impl BlockStorage {
    pub fn new() -> Self {
        Self {
            blocks: std::collections::HashMap::new(),
            block_hashes: std::collections::HashMap::new(),
        }
    }

    pub async fn store_block(&mut self, block: Block) -> StorageResult<()> {
        let height = block.header.height;
        let hash = block.header.block_hash;
        
        self.blocks.insert(height, block);
        self.block_hashes.insert(hash, height);
        
        Ok(())
    }

    pub async fn get_block(&self, height: u64) -> StorageResult<Option<Block>> {
        Ok(self.blocks.get(&height).cloned())
    }

    pub async fn get_block_by_hash(&self, hash: &[u8; 32]) -> StorageResult<Option<Block>> {
        if let Some(height) = self.block_hashes.get(hash) {
            Ok(self.blocks.get(height).cloned())
        } else {
            Ok(None)
        }
    }

    pub async fn get_latest_block(&self) -> StorageResult<Option<Block>> {
        Ok(self.blocks.values().max_by_key(|b| b.header.height).cloned())
    }
}

impl StorageComponent for BlockStorage {
    async fn initialize(&mut self) -> StorageResult<()> {
        Ok(())
    }

    async fn shutdown(&mut self) -> StorageResult<()> {
        Ok(())
    }

    async fn get_stats(&self) -> StorageResult<StorageStats> {
        Ok(StorageStats {
            total_blocks: self.blocks.len() as u64,
            total_transactions: 0, // 需要从交易存储获取
            total_size: 0, // 需要计算实际大小
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }
}
