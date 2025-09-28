// 状态管理模块
use serde::{Serialize, Deserialize};
use crate::core::{Result, BlockchainError};
use std::collections::HashMap;
// use std::sync::Arc;
// use tokio::sync::RwLock;

/// 区块链状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    /// 状态根
    pub state_root: [u8; 32],
    
    /// 最新区块哈希
    pub latest_block_hash: [u8; 32],
    
    /// 最新区块高度
    pub latest_block_height: u64,
    
    /// 账户余额
    pub balances: HashMap<String, u64>,
    
    /// 账户nonce
    pub nonces: HashMap<String, u64>,
    
    /// 合约状态
    pub contract_states: HashMap<String, ContractState>,
    
    /// 存储数据
    pub storage: HashMap<String, Vec<u8>>,
}

/// 合约状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractState {
    /// 合约地址
    pub address: String,
    
    /// 合约代码
    pub code: Vec<u8>,
    
    /// 合约存储
    pub storage: HashMap<String, Vec<u8>>,
    
    /// 合约余额
    pub balance: u64,
}

/// 状态变更
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateChange {
    /// 变更类型
    pub change_type: StateChangeType,
    
    /// 键
    pub key: StateKey,
    
    /// 值
    pub value: StateValue,
    
    /// 区块高度
    pub block_height: u64,
    
    /// 交易哈希
    pub tx_hash: [u8; 32],
}

/// 状态变更类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateChangeType {
    /// 设置余额
    SetBalance,
    
    /// 增加余额
    AddBalance,
    
    /// 减少余额
    SubtractBalance,
    
    /// 设置nonce
    SetNonce,
    
    /// 增加nonce
    IncrementNonce,
    
    /// 设置存储
    SetStorage,
    
    /// 删除存储
    DeleteStorage,
    
    /// 设置合约状态
    SetContractState,
    
    /// 删除合约状态
    DeleteContractState,
}

/// 状态键
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StateKey {
    /// 余额键
    Balance(String),
    
    /// Nonce键
    Nonce(String),
    
    /// 存储键
    Storage(String, String),
    
    /// 合约状态键
    ContractState(String),
}

/// 状态值
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateValue {
    /// 数值
    Number(u64),
    
    /// 字节数组
    Bytes(Vec<u8>),
    
    /// 字符串
    String(String),
    
    /// 合约状态
    Contract(ContractState),
}

impl State {
    /// 创建新状态
    pub fn new() -> Self {
        Self {
            state_root: [0u8; 32],
            latest_block_hash: [0u8; 32],
            latest_block_height: 0,
            balances: HashMap::new(),
            nonces: HashMap::new(),
            contract_states: HashMap::new(),
            storage: HashMap::new(),
        }
    }
    
    /// 获取账户余额
    pub async fn get_balance(&self, address: &str) -> Result<u64> {
        Ok(self.balances.get(address).copied().unwrap_or(0))
    }
    
    /// 设置账户余额
    pub async fn set_balance(&mut self, address: &str, balance: u64) -> Result<()> {
        self.balances.insert(address.to_string(), balance);
        self.update_state_root();
        Ok(())
    }
    
    /// 增加账户余额
    pub async fn add_balance(&mut self, address: &str, amount: u64) -> Result<()> {
        let current_balance = self.get_balance(address).await?;
        self.set_balance(address, current_balance + amount).await?;
        Ok(())
    }
    
    /// 减少账户余额
    pub async fn subtract_balance(&mut self, address: &str, amount: u64) -> Result<()> {
        let current_balance = self.get_balance(address).await?;
        if current_balance < amount {
            return Err(BlockchainError::InvalidState("Insufficient balance".to_string()));
        }
        self.set_balance(address, current_balance - amount).await?;
        Ok(())
    }
    
    /// 获取账户nonce
    pub async fn get_nonce(&self, address: &str) -> Result<u64> {
        Ok(self.nonces.get(address).copied().unwrap_or(0))
    }
    
    /// 设置账户nonce
    pub async fn set_nonce(&mut self, address: &str, nonce: u64) -> Result<()> {
        self.nonces.insert(address.to_string(), nonce);
        self.update_state_root();
        Ok(())
    }
    
    /// 增加账户nonce
    pub async fn increment_nonce(&mut self, address: &str) -> Result<()> {
        let current_nonce = self.get_nonce(address).await?;
        self.set_nonce(address, current_nonce + 1).await?;
        Ok(())
    }
    
    /// 获取存储值
    pub async fn get_storage(&self, contract: &str, key: &str) -> Result<Option<Vec<u8>>> {
        let storage_key = format!("{}:{}", contract, key);
        Ok(self.storage.get(&storage_key).cloned())
    }
    
    /// 设置存储值
    pub async fn set_storage(&mut self, contract: &str, key: &str, value: Vec<u8>) -> Result<()> {
        let storage_key = format!("{}:{}", contract, key);
        self.storage.insert(storage_key, value);
        self.update_state_root();
        Ok(())
    }
    
    /// 删除存储值
    pub async fn delete_storage(&mut self, contract: &str, key: &str) -> Result<()> {
        let storage_key = format!("{}:{}", contract, key);
        self.storage.remove(&storage_key);
        self.update_state_root();
        Ok(())
    }
    
    /// 获取合约状态
    pub async fn get_contract_state(&self, address: &str) -> Result<Option<&ContractState>> {
        Ok(self.contract_states.get(address))
    }
    
    /// 设置合约状态
    pub async fn set_contract_state(&mut self, contract_state: ContractState) -> Result<()> {
        self.contract_states.insert(contract_state.address.clone(), contract_state);
        self.update_state_root();
        Ok(())
    }
    
    /// 删除合约状态
    pub async fn delete_contract_state(&mut self, address: &str) -> Result<()> {
        self.contract_states.remove(address);
        self.update_state_root();
        Ok(())
    }
    
    /// 应用状态变更
    pub async fn apply_change(&mut self, change: &StateChange) -> Result<()> {
        match &change.change_type {
            StateChangeType::SetBalance => {
                if let StateValue::Number(balance) = &change.value {
                    if let StateKey::Balance(address) = &change.key {
                        self.set_balance(address, *balance).await?;
                    }
                }
            }
            StateChangeType::AddBalance => {
                if let StateValue::Number(amount) = &change.value {
                    if let StateKey::Balance(address) = &change.key {
                        self.add_balance(address, *amount).await?;
                    }
                }
            }
            StateChangeType::SubtractBalance => {
                if let StateValue::Number(amount) = &change.value {
                    if let StateKey::Balance(address) = &change.key {
                        self.subtract_balance(address, *amount).await?;
                    }
                }
            }
            StateChangeType::SetNonce => {
                if let StateValue::Number(nonce) = &change.value {
                    if let StateKey::Nonce(address) = &change.key {
                        self.set_nonce(address, *nonce).await?;
                    }
                }
            }
            StateChangeType::IncrementNonce => {
                if let StateKey::Nonce(address) = &change.key {
                    self.increment_nonce(address).await?;
                }
            }
            StateChangeType::SetStorage => {
                if let StateValue::Bytes(value) = &change.value {
                    if let StateKey::Storage(contract, key) = &change.key {
                        self.set_storage(contract, key, value.clone()).await?;
                    }
                }
            }
            StateChangeType::DeleteStorage => {
                if let StateKey::Storage(contract, key) = &change.key {
                    self.delete_storage(contract, key).await?;
                }
            }
            StateChangeType::SetContractState => {
                if let StateValue::Contract(contract_state) = &change.value {
                    self.set_contract_state(contract_state.clone()).await?;
                }
            }
            StateChangeType::DeleteContractState => {
                if let StateKey::ContractState(address) = &change.key {
                    self.delete_contract_state(address).await?;
                }
            }
        }
        
        Ok(())
    }
    
    /// 更新状态根
    fn update_state_root(&mut self) {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        
        // 哈希余额
        for (address, balance) in &self.balances {
            hasher.update(address.as_bytes());
            hasher.update(&balance.to_be_bytes());
        }
        
        // 哈希nonce
        for (address, nonce) in &self.nonces {
            hasher.update(address.as_bytes());
            hasher.update(&nonce.to_be_bytes());
        }
        
        // 哈希存储
        for (key, value) in &self.storage {
            hasher.update(key.as_bytes());
            hasher.update(value);
        }
        
        // 哈希合约状态
        for (address, contract_state) in &self.contract_states {
            hasher.update(address.as_bytes());
            hasher.update(&contract_state.serialize().unwrap_or_default());
        }
        
        self.state_root = hasher.finalize().into();
    }
    
    /// 设置最新区块哈希
    pub fn set_latest_block_hash(&mut self, block_hash: [u8; 32]) {
        self.latest_block_hash = block_hash;
    }
    
    /// 设置最新区块高度
    pub fn set_latest_block_height(&mut self, height: u64) {
        self.latest_block_height = height;
    }
    
    /// 更新状态根
    pub fn update_state_root_with_hash(&mut self, block_hash: [u8; 32]) {
        self.state_root = block_hash;
    }
    
    /// 获取状态根
    pub fn get_state_root(&self) -> [u8; 32] {
        self.state_root
    }
    
    /// 获取最新区块哈希
    pub fn get_latest_block_hash(&self) -> [u8; 32] {
        self.latest_block_hash
    }
    
    /// 获取最新区块高度
    pub fn get_latest_block_height(&self) -> u64 {
        self.latest_block_height
    }
    
    /// 序列化状态
    pub fn serialize(&self) -> Result<Vec<u8>> {
        bincode::serialize(self)
            .map_err(|e| BlockchainError::InvalidState(format!("Serialization failed: {}", e)))
    }
    
    /// 反序列化状态
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        bincode::deserialize(data)
            .map_err(|e| BlockchainError::InvalidState(format!("Deserialization failed: {}", e)))
    }
    
    /// 创建状态快照
    pub fn create_snapshot(&self) -> Self {
        self.clone()
    }
    
    /// 从快照恢复状态
    pub fn restore_from_snapshot(&mut self, snapshot: Self) {
        *self = snapshot;
    }
}

impl ContractState {
    /// 创建新合约状态
    pub fn new(address: String, code: Vec<u8>) -> Self {
        Self {
            address,
            code,
            storage: HashMap::new(),
            balance: 0,
        }
    }
    
    /// 获取存储值
    pub fn get_storage(&self, key: &str) -> Option<&Vec<u8>> {
        self.storage.get(key)
    }
    
    /// 设置存储值
    pub fn set_storage(&mut self, key: String, value: Vec<u8>) {
        self.storage.insert(key, value);
    }
    
    /// 删除存储值
    pub fn delete_storage(&mut self, key: &str) {
        self.storage.remove(key);
    }
    
    /// 序列化合约状态
    pub fn serialize(&self) -> Result<Vec<u8>> {
        bincode::serialize(self)
            .map_err(|e| BlockchainError::InvalidState(format!("Contract serialization failed: {}", e)))
    }
    
    /// 反序列化合约状态
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        bincode::deserialize(data)
            .map_err(|e| BlockchainError::InvalidState(format!("Contract deserialization failed: {}", e)))
    }
}

impl StateChange {
    /// 创建新的状态变更
    pub fn new(
        change_type: StateChangeType,
        key: StateKey,
        value: StateValue,
        block_height: u64,
        tx_hash: [u8; 32],
    ) -> Self {
        Self {
            change_type,
            key,
            value,
            block_height,
            tx_hash,
        }
    }
    
    /// 序列化状态变更
    pub fn serialize(&self) -> Result<Vec<u8>> {
        bincode::serialize(self)
            .map_err(|e| BlockchainError::InvalidState(format!("Change serialization failed: {}", e)))
    }
    
    /// 反序列化状态变更
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        bincode::deserialize(data)
            .map_err(|e| BlockchainError::InvalidState(format!("Change deserialization failed: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_state_balance_operations() {
        let mut state = State::new();
        
        // 测试设置余额
        state.set_balance("address1", 1000).await.unwrap();
        assert_eq!(state.get_balance("address1").await.unwrap(), 1000);
        
        // 测试增加余额
        state.add_balance("address1", 500).await.unwrap();
        assert_eq!(state.get_balance("address1").await.unwrap(), 1500);
        
        // 测试减少余额
        state.subtract_balance("address1", 200).await.unwrap();
        assert_eq!(state.get_balance("address1").await.unwrap(), 1300);
        
        // 测试余额不足
        assert!(state.subtract_balance("address1", 2000).await.is_err());
    }
    
    #[tokio::test]
    async fn test_state_nonce_operations() {
        let mut state = State::new();
        
        // 测试设置nonce
        state.set_nonce("address1", 5).await.unwrap();
        assert_eq!(state.get_nonce("address1").await.unwrap(), 5);
        
        // 测试增加nonce
        state.increment_nonce("address1").await.unwrap();
        assert_eq!(state.get_nonce("address1").await.unwrap(), 6);
    }
    
    #[tokio::test]
    async fn test_state_storage_operations() {
        let mut state = State::new();
        
        // 测试设置存储
        state.set_storage("contract1", "key1", vec![1, 2, 3]).await.unwrap();
        assert_eq!(state.get_storage("contract1", "key1").await.unwrap(), Some(vec![1, 2, 3]));
        
        // 测试删除存储
        state.delete_storage("contract1", "key1").await.unwrap();
        assert_eq!(state.get_storage("contract1", "key1").await.unwrap(), None);
    }
    
    #[tokio::test]
    async fn test_state_change_application() {
        let mut state = State::new();
        
        // 创建状态变更
        let change = StateChange::new(
            StateChangeType::SetBalance,
            StateKey::Balance("address1".to_string()),
            StateValue::Number(1000),
            1,
            [1u8; 32],
        );
        
        // 应用状态变更
        state.apply_change(&change).await.unwrap();
        assert_eq!(state.get_balance("address1").await.unwrap(), 1000);
    }
}
