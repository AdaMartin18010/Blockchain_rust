//! 交易存储实现

use super::{StorageComponent, StorageResult, StorageStats};
use crate::core::{Transaction};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 交易存储实现
#[derive(Debug)]
pub struct TransactionStorage {
    /// 交易哈希到交易的映射
    transactions: Arc<RwLock<HashMap<[u8; 32], Transaction>>>,
    /// 地址到交易哈希的映射
    address_to_txs: Arc<RwLock<HashMap<String, HashSet<[u8; 32]>>>>,
    /// 区块高度到交易哈希的映射
    block_to_txs: Arc<RwLock<HashMap<u64, HashSet<[u8; 32]>>>>,
    /// 待确认交易池
    mempool: Arc<RwLock<HashSet<[u8; 32]>>>,
    /// 已确认交易
    confirmed_txs: Arc<RwLock<HashSet<[u8; 32]>>>,
}

impl TransactionStorage {
    pub fn new() -> Self {
        Self {
            transactions: Arc::new(RwLock::new(HashMap::new())),
            address_to_txs: Arc::new(RwLock::new(HashMap::new())),
            block_to_txs: Arc::new(RwLock::new(HashMap::new())),
            mempool: Arc::new(RwLock::new(HashSet::new())),
            confirmed_txs: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    /// 存储交易
    pub async fn store_transaction(&mut self, tx: Transaction) -> StorageResult<()> {
        let hash = tx.hash();
        let mut transactions = self.transactions.write().await;
        let mut address_to_txs = self.address_to_txs.write().await;
        
        // 存储交易
        transactions.insert(hash, tx.clone());
        
        // 更新地址索引
        for input in &tx.inputs {
            address_to_txs
                .entry(input.address.clone())
                .or_insert_with(HashSet::new)
                .insert(hash);
        }
        
        for output in &tx.outputs {
            address_to_txs
                .entry(output.address.clone())
                .or_insert_with(HashSet::new)
                .insert(hash);
        }
        
        Ok(())
    }

    /// 获取交易
    pub async fn get_transaction(&self, hash: &[u8; 32]) -> StorageResult<Option<Transaction>> {
        let transactions = self.transactions.read().await;
        Ok(transactions.get(hash).cloned())
    }

    /// 获取地址相关的交易
    pub async fn get_transactions_by_address(&self, address: &str) -> StorageResult<Vec<Transaction>> {
        let address_to_txs = self.address_to_txs.read().await;
        let transactions = self.transactions.read().await;
        
        let mut result = Vec::new();
        if let Some(tx_hashes) = address_to_txs.get(address) {
            for hash in tx_hashes {
                if let Some(tx) = transactions.get(hash) {
                    result.push(tx.clone());
                }
            }
        }
        
        Ok(result)
    }

    /// 获取区块中的交易
    pub async fn get_transactions_by_block(&self, block_height: u64) -> StorageResult<Vec<Transaction>> {
        let block_to_txs = self.block_to_txs.read().await;
        let transactions = self.transactions.read().await;
        
        let mut result = Vec::new();
        if let Some(tx_hashes) = block_to_txs.get(&block_height) {
            for hash in tx_hashes {
                if let Some(tx) = transactions.get(hash) {
                    result.push(tx.clone());
                }
            }
        }
        
        Ok(result)
    }

    /// 添加交易到内存池
    pub async fn add_to_mempool(&mut self, tx: Transaction) -> StorageResult<()> {
        let hash = tx.hash();
        self.store_transaction(tx).await?;
        
        let mut mempool = self.mempool.write().await;
        mempool.insert(hash);
        
        Ok(())
    }

    /// 从内存池移除交易
    pub async fn remove_from_mempool(&mut self, tx_hash: &[u8; 32]) -> StorageResult<()> {
        let mut mempool = self.mempool.write().await;
        mempool.remove(tx_hash);
        Ok(())
    }

    /// 获取内存池中的交易
    pub async fn get_mempool_transactions(&self) -> StorageResult<Vec<Transaction>> {
        let mempool = self.mempool.read().await;
        let transactions = self.transactions.read().await;
        
        let mut result = Vec::new();
        for hash in mempool.iter() {
            if let Some(tx) = transactions.get(hash) {
                result.push(tx.clone());
            }
        }
        
        Ok(result)
    }

    /// 确认交易（从内存池移动到已确认）
    pub async fn confirm_transaction(&mut self, tx_hash: &[u8; 32], block_height: u64) -> StorageResult<()> {
        // 从内存池移除
        self.remove_from_mempool(tx_hash).await?;
        
        // 添加到已确认交易
        let mut confirmed_txs = self.confirmed_txs.write().await;
        confirmed_txs.insert(*tx_hash);
        
        // 添加到区块索引
        let mut block_to_txs = self.block_to_txs.write().await;
        block_to_txs
            .entry(block_height)
            .or_insert_with(HashSet::new)
            .insert(*tx_hash);
        
        Ok(())
    }

    /// 检查交易是否在内存池中
    pub async fn is_in_mempool(&self, tx_hash: &[u8; 32]) -> bool {
        let mempool = self.mempool.read().await;
        mempool.contains(tx_hash)
    }

    /// 检查交易是否已确认
    pub async fn is_confirmed(&self, tx_hash: &[u8; 32]) -> bool {
        let confirmed_txs = self.confirmed_txs.read().await;
        confirmed_txs.contains(tx_hash)
    }

    /// 获取内存池大小
    pub async fn get_mempool_size(&self) -> usize {
        let mempool = self.mempool.read().await;
        mempool.len()
    }

    /// 获取已确认交易数量
    pub async fn get_confirmed_count(&self) -> usize {
        let confirmed_txs = self.confirmed_txs.read().await;
        confirmed_txs.len()
    }

    /// 清理内存池
    pub async fn clear_mempool(&mut self) -> StorageResult<()> {
        let mut mempool = self.mempool.write().await;
        mempool.clear();
        Ok(())
    }

    /// 获取交易统计信息
    pub async fn get_transaction_stats(&self) -> TransactionStats {
        let transactions = self.transactions.read().await;
        let mempool = self.mempool.read().await;
        let confirmed_txs = self.confirmed_txs.read().await;
        
        let total_size: usize = transactions.values()
            .map(|tx| tx.size())
            .sum();
        
        TransactionStats {
            total_transactions: transactions.len(),
            mempool_size: mempool.len(),
            confirmed_count: confirmed_txs.len(),
            total_size,
        }
    }

    /// 搜索交易
    pub async fn search_transactions(&self, query: &str) -> StorageResult<Vec<Transaction>> {
        let transactions = self.transactions.read().await;
        let mut result = Vec::new();
        
        for tx in transactions.values() {
            // 搜索地址
            let matches_address = tx.inputs.iter().any(|i| i.address.contains(query))
                || tx.outputs.iter().any(|o| o.address.contains(query));
            
            if matches_address {
                result.push(tx.clone());
            }
        }
        
        Ok(result)
    }

    /// 获取交易历史
    pub async fn get_transaction_history(&self, address: &str, limit: Option<usize>) -> StorageResult<Vec<Transaction>> {
        let mut transactions = self.get_transactions_by_address(address).await?;
        
        // 按时间排序（这里简化处理，实际应该按区块高度排序）
        transactions.sort_by(|a, b| b.locktime.cmp(&a.locktime));
        
        if let Some(limit) = limit {
            transactions.truncate(limit);
        }
        
        Ok(transactions)
    }
}

/// 交易统计信息
#[derive(Debug, Clone)]
pub struct TransactionStats {
    pub total_transactions: usize,
    pub mempool_size: usize,
    pub confirmed_count: usize,
    pub total_size: usize,
}

impl StorageComponent for TransactionStorage {
    async fn initialize(&mut self) -> StorageResult<()> {
        // 初始化交易存储
        Ok(())
    }

    async fn shutdown(&mut self) -> StorageResult<()> {
        // 清理资源
        Ok(())
    }

    async fn get_stats(&self) -> StorageResult<StorageStats> {
        let tx_stats = self.get_transaction_stats().await;
        
        Ok(StorageStats {
            total_blocks: 0, // 需要从区块存储获取
            total_transactions: tx_stats.total_transactions as u64,
            total_size: tx_stats.total_size as u64,
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
    use crate::core::transaction::{TxInput, TxOutput, OutPoint};

    fn create_test_transaction() -> Transaction {
        let input = TxInput::new(
            OutPoint::new([1u8; 32], 0),
            1000,
            "test_address_1".to_string(),
        );
        let output = TxOutput::new(900, "test_address_2".to_string());
        Transaction::new(vec![input], vec![output])
    }

    #[tokio::test]
    async fn test_transaction_storage() {
        let mut storage = TransactionStorage::new();
        storage.initialize().await.unwrap();
        
        let tx = create_test_transaction();
        let hash = tx.hash();
        
        // 测试存储交易
        assert!(storage.store_transaction(tx.clone()).await.is_ok());
        
        // 测试获取交易
        let retrieved = storage.get_transaction(&hash).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().hash(), hash);
    }

    #[tokio::test]
    async fn test_address_indexing() {
        let mut storage = TransactionStorage::new();
        storage.initialize().await.unwrap();
        
        let tx = create_test_transaction();
        storage.store_transaction(tx).await.unwrap();
        
        // 测试按地址查找交易
        let txs1 = storage.get_transactions_by_address("test_address_1").await.unwrap();
        let txs2 = storage.get_transactions_by_address("test_address_2").await.unwrap();
        
        assert_eq!(txs1.len(), 1);
        assert_eq!(txs2.len(), 1);
        assert_eq!(txs1[0].hash(), txs2[0].hash());
    }

    #[tokio::test]
    async fn test_mempool_operations() {
        let mut storage = TransactionStorage::new();
        storage.initialize().await.unwrap();
        
        let tx = create_test_transaction();
        let hash = tx.hash();
        
        // 添加到内存池
        assert!(storage.add_to_mempool(tx).await.is_ok());
        assert!(storage.is_in_mempool(&hash).await);
        assert_eq!(storage.get_mempool_size().await, 1);
        
        // 确认交易
        assert!(storage.confirm_transaction(&hash, 1).await.is_ok());
        assert!(!storage.is_in_mempool(&hash).await);
        assert!(storage.is_confirmed(&hash).await);
        assert_eq!(storage.get_mempool_size().await, 0);
        assert_eq!(storage.get_confirmed_count().await, 1);
    }

    #[tokio::test]
    async fn test_transaction_stats() {
        let mut storage = TransactionStorage::new();
        storage.initialize().await.unwrap();
        
        let tx = create_test_transaction();
        storage.add_to_mempool(tx).await.unwrap();
        
        let stats = storage.get_transaction_stats().await;
        assert_eq!(stats.total_transactions, 1);
        assert_eq!(stats.mempool_size, 1);
        assert_eq!(stats.confirmed_count, 0);
        assert!(stats.total_size > 0);
    }
}
