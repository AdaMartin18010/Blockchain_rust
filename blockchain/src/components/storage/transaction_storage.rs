//! 交易存储实现

use super::{StorageComponent, StorageResult, StorageStats};
use crate::core::{Transaction};

/// 交易存储实现
#[derive(Debug)]
pub struct TransactionStorage {
    transactions: std::collections::HashMap<[u8; 32], Transaction>,
}

impl TransactionStorage {
    pub fn new() -> Self {
        Self {
            transactions: std::collections::HashMap::new(),
        }
    }

    pub async fn store_transaction(&mut self, tx: Transaction) -> StorageResult<()> {
        let hash = tx.hash();
        self.transactions.insert(hash, tx);
        Ok(())
    }

    pub async fn get_transaction(&self, hash: &[u8; 32]) -> StorageResult<Option<Transaction>> {
        Ok(self.transactions.get(hash).cloned())
    }

    pub async fn get_transactions_by_address(&self, address: &str) -> StorageResult<Vec<Transaction>> {
        let mut result = Vec::new();
        for tx in self.transactions.values() {
            let involved = tx.inputs.iter().any(|i| i.address == address)
                || tx.outputs.iter().any(|o| o.address == address);
            if involved { result.push(tx.clone()); }
        }
        Ok(result)
    }
}

impl StorageComponent for TransactionStorage {
    async fn initialize(&mut self) -> StorageResult<()> {
        Ok(())
    }

    async fn shutdown(&mut self) -> StorageResult<()> {
        Ok(())
    }

    async fn get_stats(&self) -> StorageResult<StorageStats> {
        Ok(StorageStats {
            total_blocks: 0, // 需要从区块存储获取
            total_transactions: self.transactions.len() as u64,
            total_size: 0, // 需要计算实际大小
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }
}
