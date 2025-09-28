//! 智能合约模块
//! 
//! 提供智能合约相关功能

pub mod vm;
pub mod compiler;
pub mod runtime;

pub use vm::VirtualMachine;
pub use compiler::Compiler;
pub use runtime::Runtime;

use crate::core::{Result, BlockchainError};

/// 智能合约结果类型
pub type SmartContractResult<T> = Result<T>;

/// 智能合约错误类型
#[derive(Debug, thiserror::Error)]
pub enum SmartContractError {
    #[error("虚拟机错误: {0}")]
    VirtualMachineError(String),
    #[error("编译错误: {0}")]
    CompilationError(String),
    #[error("运行时错误: {0}")]
    RuntimeError(String),
}

impl From<SmartContractError> for BlockchainError {
    fn from(err: SmartContractError) -> Self {
        BlockchainError::SmartContractError(err.to_string())
    }
}
