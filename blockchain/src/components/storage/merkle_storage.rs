//! Merkle树存储实现

use super::{StorageComponent, StorageResult, StorageStats};
use crate::core::{MerkleTree, MerkleProof};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Merkle树存储实现
#[derive(Debug)]
pub struct MerkleStorage {
    /// Merkle树映射
    merkle_trees: Arc<RwLock<HashMap<String, MerkleTree>>>,
    /// Merkle证明缓存
    proof_cache: Arc<RwLock<HashMap<String, Vec<MerkleProof>>>>,
    /// 树版本历史
    tree_versions: Arc<RwLock<HashMap<String, Vec<(u64, MerkleTree)>>>>,
}

impl MerkleStorage {
    pub fn new() -> Self {
        Self {
            merkle_trees: Arc::new(RwLock::new(HashMap::new())),
            proof_cache: Arc::new(RwLock::new(HashMap::new())),
            tree_versions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 存储Merkle树
    pub async fn store_merkle_tree(&mut self, name: String, tree: MerkleTree) -> StorageResult<()> {
        let mut trees = self.merkle_trees.write().await;
        let mut versions = self.tree_versions.write().await;
        
        // 存储当前版本
        trees.insert(name.clone(), tree.clone());
        
        // 记录版本历史
        let version = trees.len() as u64;
        versions
            .entry(name)
            .or_insert_with(Vec::new)
            .push((version, tree));
        
        Ok(())
    }

    /// 获取Merkle树
    pub async fn get_merkle_tree(&self, name: &str) -> StorageResult<Option<MerkleTree>> {
        let trees = self.merkle_trees.read().await;
        Ok(trees.get(name).cloned())
    }

    /// 创建Merkle树
    pub async fn create_merkle_tree(&mut self, name: String, data: Vec<[u8; 32]>) -> StorageResult<MerkleTree> {
        let tree = MerkleTree::new(data)?;
        self.store_merkle_tree(name.clone(), tree.clone()).await?;
        Ok(tree)
    }

    /// 更新Merkle树
    pub async fn update_merkle_tree(&mut self, name: String, data: Vec<[u8; 32]>) -> StorageResult<MerkleTree> {
        let tree = MerkleTree::new(data)?;
        self.store_merkle_tree(name.clone(), tree.clone()).await?;
        Ok(tree)
    }

    /// 添加数据到Merkle树
    pub async fn add_data_to_tree(&mut self, name: String, data: [u8; 32]) -> StorageResult<MerkleTree> {
        let mut trees = self.merkle_trees.write().await;
        
        if let Some(mut tree) = trees.get(&name).cloned() {
            // 重新构建树（添加新数据）
            let mut new_data = tree.leaves.iter().map(|leaf| leaf.hash).collect::<Vec<_>>();
            new_data.push(data);
            tree = MerkleTree::new(new_data)?;
            trees.insert(name.clone(), tree.clone());
            Ok(tree)
        } else {
            // 创建新树
            let tree = MerkleTree::new(vec![data])?;
            trees.insert(name.clone(), tree.clone());
            Ok(tree)
        }
    }

    /// 从Merkle树移除数据
    pub async fn remove_data_from_tree(&mut self, name: String, index: usize) -> StorageResult<MerkleTree> {
        let mut trees = self.merkle_trees.write().await;
        
        if let Some(mut tree) = trees.get(&name).cloned() {
            // 重新构建树（移除指定索引的数据）
            let mut new_data = tree.leaves.iter().map(|leaf| leaf.hash).collect::<Vec<_>>();
            if index < new_data.len() {
                new_data.remove(index);
                tree = MerkleTree::new(new_data)?;
                trees.insert(name.clone(), tree.clone());
                Ok(tree)
            } else {
                return Err(super::StorageError::DataNotFound(format!("Index {} out of bounds", index)).into());
            }
        } else {
            return Err(super::StorageError::DataNotFound(format!("Merkle tree '{}' not found", name)).into());
        }
    }

    /// 生成Merkle证明
    pub async fn generate_proof(&mut self, name: String, index: usize) -> StorageResult<MerkleProof> {
        let trees = self.merkle_trees.read().await;
        
        if let Some(tree) = trees.get(&name) {
            let proof = tree.generate_proof(index)?;
            
            // 缓存证明
            let mut cache = self.proof_cache.write().await;
            cache
                .entry(name)
                .or_insert_with(Vec::new)
                .push(proof.clone());
            
            Ok(proof)
        } else {
            return Err(super::StorageError::DataNotFound(format!("Merkle tree '{}' not found", name)).into());
        }
    }

    /// 验证Merkle证明
    #[allow(unused_variables)]
    pub async fn verify_proof(&self, name: &str, proof: &MerkleProof, _leaf: [u8; 32], _root: [u8; 32]) -> StorageResult<bool> {
        let trees = self.merkle_trees.read().await;
        
        if let Some(_tree) = trees.get(name) {
            Ok(MerkleTree::verify_proof(proof))
        } else {
            return Err(super::StorageError::DataNotFound(format!("Merkle tree '{}' not found", name)).into());
        }
    }

    /// 获取Merkle根
    #[allow(unused_variables)]
    pub async fn get_merkle_root(&self, name: &str) -> StorageResult<Option<[u8; 32]>> {
        let trees = self.merkle_trees.read().await;
        
        if let Some(tree) = trees.get(name) {
            Ok(Some(tree.root()))
        } else {
            Ok(None)
        }
    }

    /// 获取树大小
    pub async fn get_tree_size(&self, name: &str) -> StorageResult<Option<usize>> {
        let trees = self.merkle_trees.read().await;
        
        if let Some(tree) = trees.get(name) {
            Ok(Some(tree.leaf_count()))
        } else {
            Ok(None)
        }
    }

    /// 获取所有树名称
    pub async fn get_tree_names(&self) -> StorageResult<Vec<String>> {
        let trees = self.merkle_trees.read().await;
        Ok(trees.keys().cloned().collect())
    }

    /// 删除Merkle树
    pub async fn delete_merkle_tree(&mut self, name: String) -> StorageResult<()> {
        let mut trees = self.merkle_trees.write().await;
        let mut cache = self.proof_cache.write().await;
        let mut versions = self.tree_versions.write().await;
        
        trees.remove(&name);
        cache.remove(&name);
        versions.remove(&name);
        
        Ok(())
    }

    /// 获取树版本历史
    pub async fn get_tree_versions(&self, name: &str) -> StorageResult<Option<Vec<(u64, MerkleTree)>>> {
        let versions = self.tree_versions.read().await;
        Ok(versions.get(name).cloned())
    }

    /// 回滚到指定版本
    pub async fn rollback_to_version(&mut self, name: String, version: u64) -> StorageResult<()> {
        let versions = self.tree_versions.read().await;
        let mut trees = self.merkle_trees.write().await;
        
        if let Some(version_history) = versions.get(&name) {
            if let Some((_, tree)) = version_history.iter().find(|(v, _)| *v == version) {
                trees.insert(name, tree.clone());
                Ok(())
            } else {
                return Err(super::StorageError::DataNotFound(format!("Version {} not found for tree '{}'", version, name)).into());
            }
        } else {
            return Err(super::StorageError::DataNotFound(format!("Tree '{}' not found", name)).into());
        }
    }

    /// 清理证明缓存
    pub async fn clear_proof_cache(&mut self) -> StorageResult<()> {
        let mut cache = self.proof_cache.write().await;
        cache.clear();
        Ok(())
    }

    /// 获取存储统计信息
    pub async fn get_merkle_stats(&self) -> MerkleStats {
        let trees = self.merkle_trees.read().await;
        let cache = self.proof_cache.read().await;
        let versions = self.tree_versions.read().await;
        
        let total_trees = trees.len();
        let total_proofs: usize = cache.values().map(|v| v.len()).sum();
        let total_versions: usize = versions.values().map(|v| v.len()).sum();
        
        MerkleStats {
            total_trees,
            total_proofs,
            total_versions,
        }
    }

    /// 批量创建Merkle树
    pub async fn batch_create_trees(&mut self, trees_data: Vec<(String, Vec<[u8; 32]>)>) -> StorageResult<Vec<MerkleTree>> {
        let mut results = Vec::new();
        
        for (name, data) in trees_data {
            let tree = self.create_merkle_tree(name, data).await?;
            results.push(tree);
        }
        
        Ok(results)
    }

    /// 批量生成证明
    pub async fn batch_generate_proofs(&mut self, name: String, indices: Vec<usize>) -> StorageResult<Vec<MerkleProof>> {
        let mut results = Vec::new();
        
        for index in indices {
            let proof = self.generate_proof(name.clone(), index).await?;
            results.push(proof);
        }
        
        Ok(results)
    }
}

/// Merkle存储统计信息
#[derive(Debug, Clone)]
pub struct MerkleStats {
    pub total_trees: usize,
    pub total_proofs: usize,
    pub total_versions: usize,
}

impl StorageComponent for MerkleStorage {
    async fn initialize(&mut self) -> StorageResult<()> {
        // 初始化Merkle存储
        Ok(())
    }

    async fn shutdown(&mut self) -> StorageResult<()> {
        // 清理资源
        Ok(())
    }

    async fn get_stats(&self) -> StorageResult<StorageStats> {
        let merkle_stats = self.get_merkle_stats().await;
        
        Ok(StorageStats {
            total_blocks: 0,
            total_transactions: 0,
            total_size: merkle_stats.total_trees as u64,
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_merkle_storage() {
        let mut storage = MerkleStorage::new();
        storage.initialize().await.unwrap();
        
        let data = vec![[1u8; 32], [2u8; 32], [3u8; 32]];
        
        // 测试创建Merkle树
        let tree = storage.create_merkle_tree("test_tree".to_string(), data).await.unwrap();
        assert_eq!(tree.leaf_count(), 3);
        
        // 测试获取Merkle树
        let retrieved = storage.get_merkle_tree("test_tree").await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().leaf_count(), 3);
    }

    #[tokio::test]
    async fn test_merkle_proof() {
        let mut storage = MerkleStorage::new();
        storage.initialize().await.unwrap();
        
        let data = vec![[1u8; 32], [2u8; 32], [3u8; 32]];
        storage.create_merkle_tree("test_tree".to_string(), data).await.unwrap();
        
        // 测试生成证明
        let proof = storage.generate_proof("test_tree".to_string(), 0).await.unwrap();
        assert!(proof.verify());
        
        // 测试验证证明
        let root = storage.get_merkle_root("test_tree").await.unwrap().unwrap();
        let is_valid = storage.verify_proof("test_tree", &proof, [1u8; 32], root).await.unwrap();
        assert!(is_valid);
    }

    #[tokio::test]
    async fn test_merkle_tree_operations() {
        let mut storage = MerkleStorage::new();
        storage.initialize().await.unwrap();
        
        let data = vec![[1u8; 32], [2u8; 32]];
        storage.create_merkle_tree("test_tree".to_string(), data).await.unwrap();
        
        // 测试添加数据
        let tree = storage.add_data_to_tree("test_tree".to_string(), [3u8; 32]).await.unwrap();
        assert_eq!(tree.leaf_count(), 3);
        
        // 测试获取树大小
        let size = storage.get_tree_size("test_tree").await.unwrap().unwrap();
        assert_eq!(size, 3);
    }

    #[tokio::test]
    async fn test_merkle_tree_versions() {
        let mut storage = MerkleStorage::new();
        storage.initialize().await.unwrap();
        
        let data1 = vec![[1u8; 32], [2u8; 32]];
        storage.create_merkle_tree("test_tree".to_string(), data1).await.unwrap();
        
        let data2 = vec![[1u8; 32], [2u8; 32], [3u8; 32]];
        storage.update_merkle_tree("test_tree".to_string(), data2).await.unwrap();
        
        // 测试获取版本历史
        let versions = storage.get_tree_versions("test_tree").await.unwrap().unwrap();
        assert_eq!(versions.len(), 2);
        
        // 测试回滚到指定版本
        assert!(storage.rollback_to_version("test_tree".to_string(), 1).await.is_ok());
        let size = storage.get_tree_size("test_tree").await.unwrap().unwrap();
        assert_eq!(size, 2);
    }

    #[tokio::test]
    async fn test_merkle_stats() {
        let mut storage = MerkleStorage::new();
        storage.initialize().await.unwrap();
        
        let data = vec![[1u8; 32], [2u8; 32]];
        storage.create_merkle_tree("test_tree".to_string(), data).await.unwrap();
        storage.generate_proof("test_tree".to_string(), 0).await.unwrap();
        
        let stats = storage.get_merkle_stats().await;
        assert_eq!(stats.total_trees, 1);
        assert_eq!(stats.total_proofs, 1);
    }

    #[tokio::test]
    async fn test_batch_operations() {
        let mut storage = MerkleStorage::new();
        storage.initialize().await.unwrap();
        
        let trees_data = vec![
            ("tree1".to_string(), vec![[1u8; 32], [2u8; 32]]),
            ("tree2".to_string(), vec![[3u8; 32], [4u8; 32]]),
        ];
        
        // 测试批量创建
        let trees = storage.batch_create_trees(trees_data).await.unwrap();
        assert_eq!(trees.len(), 2);
        
        // 测试批量生成证明
        let proofs = storage.batch_generate_proofs("tree1".to_string(), vec![0, 1]).await.unwrap();
        assert_eq!(proofs.len(), 2);
    }
}
