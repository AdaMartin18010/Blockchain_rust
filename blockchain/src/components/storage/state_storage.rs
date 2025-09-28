//! 状态存储实现

use super::{StorageComponent, StorageResult, StorageStats};
// use crate::core::{Result, BlockchainError};

/// 状态存储实现
#[derive(Debug)]
pub struct StateStorage {
    state: std::collections::HashMap<String, String>,
}

impl StateStorage {
    pub fn new() -> Self {
        Self {
            state: std::collections::HashMap::new(),
        }
    }

    pub async fn set_state(&mut self, key: String, value: String) -> StorageResult<()> {
        self.state.insert(key, value);
        Ok(())
    }

    pub async fn get_state(&self, key: &str) -> StorageResult<Option<String>> {
        Ok(self.state.get(key).cloned())
    }

    pub async fn delete_state(&mut self, key: &str) -> StorageResult<()> {
        self.state.remove(key);
        Ok(())
    }

    pub async fn get_all_state(&self) -> StorageResult<std::collections::HashMap<String, String>> {
        Ok(self.state.clone())
    }
}

impl StorageComponent for StateStorage {
    fn initialize(&mut self) -> impl std::future::Future<Output = StorageResult<()>> + Send {
        async move { Ok(()) }
    }

    fn shutdown(&mut self) -> impl std::future::Future<Output = StorageResult<()>> + Send {
        async move { Ok(()) }
    }

    fn get_stats(&self) -> impl std::future::Future<Output = StorageResult<StorageStats>> + Send {
        let state_len = self.state.len() as u64;
        async move {
            Ok(StorageStats {
                total_blocks: 0,
                total_transactions: 0,
                total_size: state_len,
                last_updated: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            })
        }
    }
}
