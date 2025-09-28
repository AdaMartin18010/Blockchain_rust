//! 基础设施层模块
//! 
//! 提供区块链基础设施，包括数据库、缓存、日志、监控等

pub mod database;
pub mod cache;
pub mod logging;
pub mod monitoring;

pub use database::Database;
pub use cache::Cache;
pub use logging::Logging;
pub use monitoring::Monitoring;

use crate::core::{Result, BlockchainError};

/// 基础设施层结果类型
pub type InfrastructureResult<T> = Result<T>;

/// 基础设施层错误类型
#[derive(Debug, thiserror::Error)]
pub enum InfrastructureError {
    #[error("数据库错误: {0}")]
    DatabaseError(String),
    #[error("缓存错误: {0}")]
    CacheError(String),
    #[error("日志错误: {0}")]
    LoggingError(String),
    #[error("监控错误: {0}")]
    MonitoringError(String),
}

impl From<InfrastructureError> for BlockchainError {
    fn from(err: InfrastructureError) -> Self {
        BlockchainError::InfrastructureError(err.to_string())
    }
}
