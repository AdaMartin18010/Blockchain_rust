//! 存储组件模块
//! 
//! 提供区块链数据存储功能，包括区块存储、交易存储、状态存储等

pub mod block_storage;
pub mod transaction_storage;
pub mod state_storage;
pub mod merkle_storage;

pub use block_storage::BlockStorage;
pub use transaction_storage::TransactionStorage;
pub use state_storage::StateStorage;
pub use merkle_storage::MerkleStorage;

use crate::core::{Result, BlockchainError};

/// 存储组件结果类型
pub type StorageResult<T> = Result<T>;

/// 存储组件错误类型
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("存储操作失败: {0}")]
    StorageFailed(String),
    #[error("数据不存在: {0}")]
    DataNotFound(String),
    #[error("序列化失败: {0}")]
    SerializationFailed(String),
    #[error("反序列化失败: {0}")]
    DeserializationFailed(String),
}

impl From<StorageError> for BlockchainError {
    fn from(err: StorageError) -> Self {
        BlockchainError::StorageError(err.to_string())
    }
}

/// 存储组件接口
pub trait StorageComponent {
    /// 初始化存储
    fn initialize(&mut self) -> impl std::future::Future<Output = StorageResult<()>> + Send;
    
    /// 关闭存储
    fn shutdown(&mut self) -> impl std::future::Future<Output = StorageResult<()>> + Send;
    
    /// 获取存储统计信息
    fn get_stats(&self) -> impl std::future::Future<Output = StorageResult<StorageStats>> + Send;
}

/// 存储统计信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StorageStats {
    pub total_blocks: u64,
    pub total_transactions: u64,
    pub total_size: u64,
    pub last_updated: u64,
}
