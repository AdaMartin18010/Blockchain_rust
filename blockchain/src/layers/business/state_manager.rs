//! 状态管理器实现

use super::BusinessResult;
// use crate::core::{Result, BlockchainError};

/// 状态管理器
#[derive(Debug)]
pub struct StateManager {
    state: std::collections::HashMap<String, String>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            state: std::collections::HashMap::new(),
        }
    }

    pub async fn set_state(&mut self, key: String, value: String) -> BusinessResult<()> {
        self.state.insert(key, value);
        Ok(())
    }

    pub async fn get_state(&self, key: &str) -> BusinessResult<Option<String>> {
        Ok(self.state.get(key).cloned())
    }

    pub async fn delete_state(&mut self, key: &str) -> BusinessResult<()> {
        self.state.remove(key);
        Ok(())
    }

    pub async fn get_all_state(&self) -> BusinessResult<std::collections::HashMap<String, String>> {
        Ok(self.state.clone())
    }

    pub async fn commit_state(&mut self) -> BusinessResult<()> {
        // 提交状态变更
        // 这里需要实际的状态持久化逻辑
        Ok(())
    }

    pub async fn rollback_state(&mut self) -> BusinessResult<()> {
        // 回滚状态变更
        // 这里需要实际的状态回滚逻辑
        Ok(())
    }
}
