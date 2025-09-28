//! 算法模块
//! 
//! 提供区块链相关的算法实现

pub mod consensus_algorithms;
pub mod cryptographic_algorithms;
pub mod optimization_algorithms;

pub use consensus_algorithms::ConsensusAlgorithms;
pub use cryptographic_algorithms::CryptographicAlgorithms;
pub use optimization_algorithms::OptimizationAlgorithms;

use crate::core::{Result, BlockchainError};

/// 算法结果类型
pub type AlgorithmResult<T> = Result<T>;

/// 算法错误类型
#[derive(Debug, thiserror::Error)]
pub enum AlgorithmError {
    #[error("算法执行失败: {0}")]
    AlgorithmExecutionFailed(String),
    #[error("参数错误: {0}")]
    InvalidParameters(String),
    #[error("计算错误: {0}")]
    ComputationError(String),
}

impl From<AlgorithmError> for BlockchainError {
    fn from(err: AlgorithmError) -> Self {
        BlockchainError::AlgorithmError(err.to_string())
    }
}
