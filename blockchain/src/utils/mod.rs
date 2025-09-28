//! 工具模块
//! 
//! 提供区块链相关的工具函数

pub mod crypto_utils;
pub mod encoding_utils;
pub mod time_utils;
pub mod validation_utils;

pub use crypto_utils::CryptoUtils;
pub use encoding_utils::EncodingUtils;
pub use time_utils::TimeUtils;
pub use validation_utils::ValidationUtils;

use crate::core::{Result, BlockchainError};

/// 工具结果类型
pub type UtilsResult<T> = Result<T>;

/// 工具错误类型
#[derive(Debug, thiserror::Error)]
pub enum UtilsError {
    #[error("工具错误: {0}")]
    UtilsError(String),
    #[error("编码错误: {0}")]
    EncodingError(String),
    #[error("验证错误: {0}")]
    ValidationError(String),
}

impl From<UtilsError> for BlockchainError {
    fn from(err: UtilsError) -> Self {
        BlockchainError::UtilsError(err.to_string())
    }
}
