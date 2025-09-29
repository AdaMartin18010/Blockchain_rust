//! 状态存储实现

use super::{StorageComponent, StorageResult, StorageStats};
use crate::core::{State, StateChange};
use std::collections::{HashMap, BTreeMap};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 状态存储实现
#[derive(Debug)]
pub struct StateStorage {
    /// 区块高度到状态的映射
    states: Arc<RwLock<BTreeMap<u64, State>>>,
    /// 区块高度到状态变更的映射
    changes: Arc<RwLock<HashMap<u64, Vec<StateChange>>>>,
    /// 当前状态
    current_state: Arc<RwLock<State>>,
    /// 状态快照
    snapshots: Arc<RwLock<HashMap<u64, State>>>,
    /// 简单键值存储（向后兼容）
    simple_state: Arc<RwLock<HashMap<String, String>>>,
}

impl StateStorage {
    pub fn new() -> Self {
        Self {
            states: Arc::new(RwLock::new(BTreeMap::new())),
            changes: Arc::new(RwLock::new(HashMap::new())),
            current_state: Arc::new(RwLock::new(State::new())),
            snapshots: Arc::new(RwLock::new(HashMap::new())),
            simple_state: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 存储状态
    pub async fn store_state(&mut self, height: u64, state: State) -> StorageResult<()> {
        let mut states = self.states.write().await;
        let mut current_state = self.current_state.write().await;
        
        states.insert(height, state.clone());
        *current_state = state;
        
        Ok(())
    }

    /// 获取状态
    pub async fn get_state(&self, height: u64) -> StorageResult<Option<State>> {
        let states = self.states.read().await;
        Ok(states.get(&height).cloned())
    }

    /// 获取当前状态
    pub async fn get_current_state(&self) -> StorageResult<State> {
        let current_state = self.current_state.read().await;
        Ok(current_state.clone())
    }

    /// 存储状态变更
    pub async fn store_changes(&mut self, height: u64, changes: Vec<StateChange>) -> StorageResult<()> {
        let mut changes_map = self.changes.write().await;
        changes_map.insert(height, changes);
        Ok(())
    }

    /// 获取状态变更
    pub async fn get_changes(&self, height: u64) -> StorageResult<Option<Vec<StateChange>>> {
        let changes = self.changes.read().await;
        Ok(changes.get(&height).cloned())
    }

    /// 应用状态变更
    pub async fn apply_changes(&mut self, changes: Vec<StateChange>) -> StorageResult<()> {
        let mut current_state = self.current_state.write().await;
        
        for change in changes {
            current_state.apply_change(&change).await?;
        }
        
        Ok(())
    }

    /// 回滚到指定高度
    pub async fn rollback_to_height(&mut self, height: u64) -> StorageResult<()> {
        let states = self.states.read().await;
        let mut current_state = self.current_state.write().await;
        
        if let Some(state) = states.get(&height) {
            *current_state = state.clone();
        } else {
            return Err(super::StorageError::DataNotFound(format!("State at height {} not found", height)).into());
        }
        
        Ok(())
    }

    /// 创建状态快照
    pub async fn create_snapshot(&mut self, height: u64) -> StorageResult<()> {
        let current_state = self.current_state.read().await;
        let mut snapshots = self.snapshots.write().await;
        
        snapshots.insert(height, current_state.clone());
        Ok(())
    }

    /// 从快照恢复
    pub async fn restore_from_snapshot(&mut self, height: u64) -> StorageResult<()> {
        let snapshots = self.snapshots.read().await;
        let mut current_state = self.current_state.write().await;
        
        if let Some(snapshot) = snapshots.get(&height) {
            *current_state = snapshot.clone();
        } else {
            return Err(super::StorageError::DataNotFound(format!("Snapshot at height {} not found", height)).into());
        }
        
        Ok(())
    }

    /// 获取状态历史
    #[allow(unused_variables)]
    pub async fn get_state_history(&self, start_height: u64, end_height: u64) -> StorageResult<Vec<State>> {
        let states = self.states.read().await;
        let mut result = Vec::new();
        
        for (_height, state) in states.range(start_height..=end_height) {
            result.push(state.clone());
        }
        
        Ok(result)
    }

    /// 获取状态变更历史
    pub async fn get_changes_history(&self, start_height: u64, end_height: u64) -> StorageResult<Vec<StateChange>> {
        let changes = self.changes.read().await;
        let mut result = Vec::new();
        
        for height in start_height..=end_height {
            if let Some(changes_at_height) = changes.get(&height) {
                result.extend(changes_at_height.clone());
            }
        }
        
        Ok(result)
    }

    /// 获取账户余额
    pub async fn get_balance(&self, address: &str) -> StorageResult<u64> {
        let current_state = self.current_state.read().await;
        Ok(current_state.get_balance(address).await?)
    }

    /// 设置账户余额
    pub async fn set_balance(&mut self, address: &str, balance: u64) -> StorageResult<()> {
        let mut current_state = self.current_state.write().await;
        current_state.set_balance(address, balance).await?;
        Ok(())
    }

    /// 获取账户nonce
    pub async fn get_nonce(&self, address: &str) -> StorageResult<u64> {
        let current_state = self.current_state.read().await;
        Ok(current_state.get_nonce(address).await?)
    }

    /// 设置账户nonce
    pub async fn set_nonce(&mut self, address: &str, nonce: u64) -> StorageResult<()> {
        let mut current_state = self.current_state.write().await;
        current_state.set_nonce(address, nonce).await?;
        Ok(())
    }

    /// 获取存储值
    pub async fn get_storage(&self, contract: &str, key: &str) -> StorageResult<Option<Vec<u8>>> {
        let current_state = self.current_state.read().await;
        Ok(current_state.get_storage(contract, key).await?)
    }

    /// 设置存储值
    pub async fn set_storage(&mut self, contract: &str, key: &str, value: Vec<u8>) -> StorageResult<()> {
        let mut current_state = self.current_state.write().await;
        current_state.set_storage(contract, key, value).await?;
        Ok(())
    }

    /// 获取状态统计信息
    pub async fn get_state_stats(&self) -> StateStats {
        let states = self.states.read().await;
        let changes = self.changes.read().await;
        let snapshots = self.snapshots.read().await;
        let current_state = self.current_state.read().await;
        
        StateStats {
            total_states: states.len(),
            total_changes: changes.values().map(|v| v.len()).sum(),
            total_snapshots: snapshots.len(),
            current_balances: current_state.balances.len(),
            current_nonces: current_state.nonces.len(),
            current_storage: current_state.storage.len(),
        }
    }

    /// 清理旧状态
    pub async fn cleanup_old_states(&mut self, keep_count: usize) -> StorageResult<()> {
        let mut states = self.states.write().await;
        let mut changes = self.changes.write().await;
        
        if states.len() > keep_count {
            let to_remove = states.len() - keep_count;
            let keys_to_remove: Vec<u64> = states.keys().take(to_remove).cloned().collect();
            
            for key in keys_to_remove {
                states.remove(&key);
                changes.remove(&key);
            }
        }
        
        Ok(())
    }

    /// 验证状态一致性
    pub async fn verify_state_consistency(&self) -> StorageResult<bool> {
        let states = self.states.read().await;
        let changes = self.changes.read().await;
        
        // 检查每个状态是否都有对应的变更记录
        for height in states.keys() {
            if !changes.contains_key(height) {
                return Ok(false);
            }
        }
        
        Ok(true)
    }

    // 向后兼容的简单键值存储方法
    pub async fn set_state(&mut self, key: String, value: String) -> StorageResult<()> {
        let mut simple_state = self.simple_state.write().await;
        simple_state.insert(key, value);
        Ok(())
    }

    pub async fn get_state_value(&self, key: &str) -> StorageResult<Option<String>> {
        let simple_state = self.simple_state.read().await;
        Ok(simple_state.get(key).cloned())
    }

    pub async fn delete_state(&mut self, key: &str) -> StorageResult<()> {
        let mut simple_state = self.simple_state.write().await;
        simple_state.remove(key);
        Ok(())
    }

    pub async fn get_all_state(&self) -> StorageResult<HashMap<String, String>> {
        let simple_state = self.simple_state.read().await;
        Ok(simple_state.clone())
    }
}

/// 状态统计信息
#[derive(Debug, Clone)]
pub struct StateStats {
    pub total_states: usize,
    pub total_changes: usize,
    pub total_snapshots: usize,
    pub current_balances: usize,
    pub current_nonces: usize,
    pub current_storage: usize,
}

impl StorageComponent for StateStorage {
    async fn initialize(&mut self) -> StorageResult<()> {
        // 初始化状态存储
        Ok(())
    }

    async fn shutdown(&mut self) -> StorageResult<()> {
        // 清理资源
        Ok(())
    }

    async fn get_stats(&self) -> StorageResult<StorageStats> {
        let state_stats = self.get_state_stats().await;
        let simple_state = self.simple_state.read().await;
        
        Ok(StorageStats {
            total_blocks: state_stats.total_states as u64,
            total_transactions: 0, // 需要从交易存储获取
            total_size: simple_state.len() as u64,
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
    use crate::core::state::{StateChange, StateChangeType, StateKey, StateValue};

    #[tokio::test]
    async fn test_state_storage() {
        let mut storage = StateStorage::new();
        storage.initialize().await.unwrap();
        
        let mut state = State::new();
        state.set_balance("test_address", 1000).await.unwrap();
        
        // 测试存储状态
        assert!(storage.store_state(1, state).await.is_ok());
        
        // 测试获取状态
        let retrieved = storage.get_state(1).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().get_balance("test_address").await.unwrap(), 1000);
    }

    #[tokio::test]
    async fn test_state_changes() {
        let mut storage = StateStorage::new();
        storage.initialize().await.unwrap();
        
        let changes = vec![
            StateChange::new(
                StateChangeType::SetBalance,
                StateKey::Balance("test_address".to_string()),
                StateValue::Number(1000),
                1,
                [1u8; 32],
            ),
        ];
        
        // 测试存储状态变更
        assert!(storage.store_changes(1, changes.clone()).await.is_ok());
        
        // 测试获取状态变更
        let retrieved = storage.get_changes(1).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_balance_operations() {
        let mut storage = StateStorage::new();
        storage.initialize().await.unwrap();
        
        // 测试设置余额
        assert!(storage.set_balance("test_address", 1000).await.is_ok());
        assert_eq!(storage.get_balance("test_address").await.unwrap(), 1000);
        
        // 测试设置nonce
        assert!(storage.set_nonce("test_address", 5).await.is_ok());
        assert_eq!(storage.get_nonce("test_address").await.unwrap(), 5);
    }

    #[tokio::test]
    async fn test_storage_operations() {
        let mut storage = StateStorage::new();
        storage.initialize().await.unwrap();
        
        // 测试设置存储
        assert!(storage.set_storage("contract", "key", vec![1, 2, 3]).await.is_ok());
        assert_eq!(storage.get_storage("contract", "key").await.unwrap(), Some(vec![1, 2, 3]));
    }

    #[tokio::test]
    async fn test_snapshot_operations() {
        let mut storage = StateStorage::new();
        storage.initialize().await.unwrap();
        
        storage.set_balance("test_address", 1000).await.unwrap();
        
        // 创建快照
        assert!(storage.create_snapshot(1).await.is_ok());
        
        // 修改状态
        storage.set_balance("test_address", 2000).await.unwrap();
        assert_eq!(storage.get_balance("test_address").await.unwrap(), 2000);
        
        // 从快照恢复
        assert!(storage.restore_from_snapshot(1).await.is_ok());
        assert_eq!(storage.get_balance("test_address").await.unwrap(), 1000);
    }

    #[tokio::test]
    async fn test_state_stats() {
        let mut storage = StateStorage::new();
        storage.initialize().await.unwrap();
        
        storage.set_balance("test_address", 1000).await.unwrap();
        storage.store_state(1, State::new()).await.unwrap();
        
        let stats = storage.get_state_stats().await;
        assert_eq!(stats.total_states, 1);
        assert_eq!(stats.current_balances, 1);
    }

    #[tokio::test]
    async fn test_simple_state_compatibility() {
        let mut storage = StateStorage::new();
        storage.initialize().await.unwrap();
        
        // 测试简单状态存储
        assert!(storage.set_state("key1".to_string(), "value1".to_string()).await.is_ok());
        assert_eq!(storage.get_state_value("key1").await.unwrap(), Some("value1".to_string()));
        
        // 测试删除状态
        assert!(storage.delete_state("key1").await.is_ok());
        assert_eq!(storage.get_state_value("key1").await.unwrap(), None);
    }
}
