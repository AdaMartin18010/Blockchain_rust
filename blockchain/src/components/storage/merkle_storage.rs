//! Merkle树存储实现

use super::{StorageComponent, StorageResult, StorageStats};
use crate::core::{MerkleTree};

/// Merkle树存储实现
#[derive(Debug)]
pub struct MerkleStorage {
    merkle_trees: std::collections::HashMap<String, MerkleTree>,
}

impl MerkleStorage {
    pub fn new() -> Self {
        Self {
            merkle_trees: std::collections::HashMap::new(),
        }
    }

    pub async fn store_merkle_tree(&mut self, name: String, tree: MerkleTree) -> StorageResult<()> {
        self.merkle_trees.insert(name, tree);
        Ok(())
    }

    pub async fn get_merkle_tree(&self, name: &str) -> StorageResult<Option<MerkleTree>> {
        Ok(self.merkle_trees.get(name).cloned())
    }

    pub async fn create_merkle_tree(&mut self, name: String, data: Vec<[u8; 32]>) -> StorageResult<MerkleTree> {
        let tree = MerkleTree::new(data)?;
        self.merkle_trees.insert(name.clone(), tree.clone());
        Ok(tree)
    }
}

impl StorageComponent for MerkleStorage {
    async fn initialize(&mut self) -> StorageResult<()> {
        Ok(())
    }

    async fn shutdown(&mut self) -> StorageResult<()> {
        Ok(())
    }

    async fn get_stats(&self) -> StorageResult<StorageStats> {
        Ok(StorageStats {
            total_blocks: 0,
            total_transactions: 0,
            total_size: self.merkle_trees.len() as u64,
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }
}
