//! 业务逻辑层模块
//! 
//! 提供区块链业务逻辑，包括交易处理、智能合约执行、状态管理等

pub mod transaction_processor;
pub mod smart_contract_engine;
pub mod state_manager;
pub mod wallet_manager;

pub use transaction_processor::TransactionProcessor;
pub use smart_contract_engine::SmartContractEngine;
pub use state_manager::StateManager;
pub use wallet_manager::WalletManager;

use crate::core::{Result, BlockchainError};

/// 业务逻辑层结果类型
pub type BusinessResult<T> = Result<T>;

/// 业务逻辑层错误类型
#[derive(Debug, thiserror::Error)]
pub enum BusinessError {
    #[error("交易处理失败: {0}")]
    TransactionProcessingFailed(String),
    #[error("智能合约执行失败: {0}")]
    SmartContractExecutionFailed(String),
    #[error("状态管理失败: {0}")]
    StateManagementFailed(String),
    #[error("钱包操作失败: {0}")]
    WalletOperationFailed(String),
}

impl From<BusinessError> for BlockchainError {
    fn from(err: BusinessError) -> Self {
        BlockchainError::BusinessError(err.to_string())
    }
}
